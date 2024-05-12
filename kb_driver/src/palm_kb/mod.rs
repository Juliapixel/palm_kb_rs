use embassy_futures::join::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_usb::class::hid::HidWriter;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Output, Pin},
    mode::Async,
    peripherals::USB_OTG_FS,
    usart::{BasicInstance, Error, UartRx},
    usb::Driver,
    Peripheral,
    PeripheralRef
};
use usbd_hid::descriptor::KeyboardReport;

use crate::{debug, error, info, warn};

use self::state::State;

pub mod matrix;
pub mod state;

static REPORT: Mutex<ThreadModeRawMutex, KeyboardReport> = Mutex::new(KeyboardReport::default());

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
async fn write_kb_report<'d>(report: &'static Mutex<ThreadModeRawMutex, KeyboardReport>, mut writer: HidWriter<'d, Driver<'d, USB_OTG_FS>, 8>) {
    loop {
        writer.ready().await;
        let report = *report.lock().await;
        match writer.write_serialize(&report).await {
            Ok(_) => (),
            Err(e) => warn!("failed to write to USB endpoint {}", e),
        }
    }
}

/// Reads the initial handshake bytes and checks if they're right
async fn read_initial_bytes<'d, T: BasicInstance>(
    uart: &mut UartRx<'d, T, Async>
) -> bool {
    let mut buf = [0u8; 2];
    let resp = uart.read(&mut buf).await;
    debug!("received initial buf: {:02x}", &buf);
    match resp {
        Ok(_) => {
            buf == [0xFA, 0xFD]
        },
        Err(_) => false,
    }
}

/// Main driver loop, manages the connection to the keyboard and stuff
async fn listen_kb<'p, T: BasicInstance>(
    report: &'static Mutex<ThreadModeRawMutex, KeyboardReport>,
    mut vcc: Output<'p>,
    mut rts: Output<'p>,
    mut dcd: ExtiInput<'p>,
    mut state: State,
    mut uart: UartRx<'p, T, Async>
) {
    // reset to initial state in case it wasn't already at it
    vcc.set_low();
    rts.set_low();

    // turn on power delivery to kb
    vcc.set_high();

    // wait for keyboard to be ready
    dcd.wait_for_rising_edge().await;
    info!("keyboard ready");

    // toggle RTS to trigger the handshake frames
    rts.set_high();

    let handshake_successful = read_initial_bytes(&mut uart).await;
    // TODO: handle this when things are stable enough to actually be able to
    // properly receive the initial frames
    if handshake_successful {
        info!("keyboard handshake successful")
    } else {
        error!("keyboard handshake unsuccessful")
    }

    loop {
        let mut buf = [0u8; 1];
        let read = uart.read(&mut buf).await;
        match read {
            Ok(_) => {
                debug!("received buf: {:08b}", buf[0]);
                state.update_from_kb_input(buf[0]);
                *report.lock().await = KeyboardReport::from(&state);
            },
            Err(Error::Framing) => warn!("UART Framing error"),
            Err(Error::BufferTooLong) => warn!("UART buffer too long for DMA"),
            Err(Error::Noise) => warn!("UART Noise error"),
            Err(Error::Overrun) => warn!("UART buffer overrun"),
            Err(Error::Parity) => warn!("UART parity bit error"),
            Err(_) => error!("UART unknown error"),
        };
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
        let input = ExtiInput::new(dcd, exti, embassy_stm32::gpio::Pull::None);
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
            embassy_stm32::gpio::Speed::Low
        );
        let rts = Output::new(
            self.rts,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::Medium
        );
        join(write_kb_report(&REPORT, self.writer), listen_kb(&REPORT, vcc, rts, self.dcd, self.state, self.uart)).await;
    }
}
