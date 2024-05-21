use core::cell::UnsafeCell;

use embassy_futures::{join::join, select::select3};
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Output, Pin},
    mode::Async,
    peripherals::USB_OTG_FS,
    usart::{BasicInstance, Error, RingBufferedUartRx, UartRx},
    usb::Driver,
    Peripheral, PeripheralRef
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, blocking_mutex::Mutex};
use embassy_time::{Duration, Ticker, Timer};
use embassy_usb::class::hid::HidWriter;
use embedded_io_async::Read;
use usbd_hid::descriptor::KeyboardReport;

use crate::{debug, error, info, warn};

use self::state::State;

pub mod matrix;
pub mod state;

static REPORT: Mutex<ThreadModeRawMutex, UnsafeCell<KeyboardReport>> =
    Mutex::new(UnsafeCell::new(KeyboardReport::default()));

/// Compatibility layer between the Palm keyboard's UART interface and USB-HID
pub struct KeyboardDriver<'d, 'u, T: BasicInstance, V: Pin, R: Pin> {
    uart: UartRx<'u, T, Async>,
    vcc: PeripheralRef<'d, V>,
    rts: PeripheralRef<'d, R>,
    dcd: ExtiInput<'d>,
    writer: HidWriter<'d, Driver<'d, USB_OTG_FS>, 8>,
    state: State
}

/// Constantly writes the current KeyboardReport out to the USB-HID endpoint
async fn write_kb_report<'d>(
    report: &'static Mutex<ThreadModeRawMutex, UnsafeCell<KeyboardReport>>,
    mut writer: HidWriter<'d, Driver<'d, USB_OTG_FS>, 8>
) {
    loop {
        writer.ready().await;
        let report = unsafe { report.lock(|r| *r.get()) };
        match writer.write_serialize(&report).await {
            Ok(_) => (),
            Err(e) => warn!("failed to write to USB endpoint {}", e)
        }
    }
}

/// Reads the initial handshake bytes and checks if they're right
async fn read_initial_bytes<'d, T: BasicInstance>(
    uart: &mut RingBufferedUartRx<'d, T>
) -> bool {
    let mut buf = [0u8; 2];
    let resp = uart.read_exact(&mut buf).await;
    debug!("received initial buf: {:02X}", &buf);
    match resp {
        Ok(_) => buf == [0xFA, 0xFD],
        Err(_) => false
    }
}

/// this method *should* be cancel safe (as of embassy-stm32@51d55309)
async fn receive_forever<'u, T: BasicInstance>(
    report: &'static Mutex<ThreadModeRawMutex, UnsafeCell<KeyboardReport>>,
    state: &mut State,
    uart: &mut RingBufferedUartRx<'u, T>
) -> ! {
    loop {
        let mut buf = [0u8; 1];
        let read = uart.read(&mut buf).await;
        match read {
            Ok(_) => {
                debug!("received buf: {:08b}", buf[0]);
                state.update_from_kb_input(buf[0]);
                unsafe { report.lock(|r| *r.get() = KeyboardReport::from(&*state)) }
            }
            Err(Error::Framing) => warn!("UART Framing error"),
            Err(Error::BufferTooLong) => warn!("UART buffer too long for DMA"),
            Err(Error::Noise) => warn!("UART Noise error"),
            Err(Error::Overrun) => warn!("UART buffer overrun"),
            Err(Error::Parity) => warn!("UART parity bit error"),
            Err(_) => error!("UART unknown error")
        };
    }
}

/// Main driver loop, manages the connection to the keyboard and stuff
async fn listen_kb<'p, T: BasicInstance>(
    report: &'static Mutex<ThreadModeRawMutex, UnsafeCell<KeyboardReport>>,
    mut vcc: Output<'p>,
    mut rts: Output<'p>,
    mut dcd: ExtiInput<'p>,
    mut state: State,
    uart: UartRx<'p, T, Async>
) {
    // reset to initial state in case it wasn't already at it
    vcc.set_low();
    rts.set_low();
    let mut ring_buffer = [0u8; 256];
    let mut uart = uart.into_ring_buffered(&mut ring_buffer);

    loop {
        // toggle RTS to trigger the handshake frames
        rts.set_low();
        Timer::after(Duration::from_millis(15)).await;
        // turn on power delivery to kb
        vcc.set_high();
        rts.set_high();

        let handshake_successful = embassy_time::with_timeout(
            Duration::from_millis(100),
            read_initial_bytes(&mut uart)
        )
        .await
        .unwrap_or(false);
        if handshake_successful {
            info!("keyboard handshake successful");
            break;
        } else {
            error!("keyboard handshake unsuccessful");
        }
    }

    // toggle RTS and perform handshake to avoid going into low-power mode
    let mut ticker = Ticker::every(Duration::from_secs(60));

    loop {
        select3(
            dcd.wait_for_rising_edge(),
            ticker.next(),
            receive_forever(report, &mut state, &mut uart)
        )
        .await;

        let mut err_count: u32 = 0;
        loop {
            rts.set_low();
            // gotta have this here or kb just will not notice the toggle
            Timer::after(Duration::from_millis(15)).await;
            rts.set_high();
            info!("keyboard reconnecting");
            let handshake_successful = embassy_time::with_timeout(
                Duration::from_millis(30),
                read_initial_bytes(&mut uart)
            )
            .await
            .unwrap_or(false);
            if handshake_successful {
                info!("keyboard handshake successful");
                break;
            } else {
                error!("keyboard handshake unsuccessful");
                err_count += 1;
                if err_count >= 5 {
                    state.reset();
                }
            }
        }
        ticker.reset();
    }
}

impl<'d, 'u, T: BasicInstance, V: Pin, R: Pin> KeyboardDriver<'d, 'u, T, V, R> {
    pub fn new<D: Pin>(
        uart: UartRx<'u, T, Async>,
        vcc: impl Peripheral<P = V> + 'd,
        rts: impl Peripheral<P = R> + 'd,
        dcd: impl Peripheral<P = D> + 'd,
        exti: impl Peripheral<P = D::ExtiChannel> + 'd,
        writer: HidWriter<'d, Driver<'d, USB_OTG_FS>, 8>
    ) -> Self {
        let input = ExtiInput::new(dcd, exti, embassy_stm32::gpio::Pull::Down);
        Self {
            uart,
            vcc: vcc.into_ref(),
            rts: rts.into_ref(),
            dcd: input,
            state: State::new(),
            writer
        }
    }

    /// Runs the driver forever
    pub async fn run(self) {
        info!("starting keyboard driver");
        let vcc = Output::new(
            self.vcc,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::VeryHigh
        );
        let rts = Output::new(
            self.rts,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::VeryHigh
        );
        join(
            write_kb_report(&REPORT, self.writer),
            listen_kb(&REPORT, vcc, rts, self.dcd, self.state, self.uart)
        )
        .await;
    }
}
