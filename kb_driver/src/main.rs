#![no_std]
#![no_main]

use kb_driver_proc_macro::{debug, error, info};
use embassy_executor::Spawner;
use embassy_futures::select::{select3, Either3};
use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{AnyPin, Output, Pin},
    peripherals::{self},
    usart::{self, Config as UsartConfig, DataBits, Parity, StopBits, UartRx},
    usb::{self, Config as UsbOtgConfig}, Config, Peripheral
};
use embassy_time::{Duration, Timer};
use embassy_usb::{
    class::hid::{Config as HidConfig, HidReaderWriter, State},
    Config as UsbConfig
};
use handlers::{MyRequestHandler, MyUsbHandler};
use kb_driver::palm_kb::KeyboardDriver;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

#[cfg(feature = "defmt")]
use defmt_rtt as _;

use panic_probe as _;

mod handlers;

bind_interrupts!(struct UsbIrq {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});

bind_interrupts!(struct UsartIrq {
    USART2 => usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        // for some reason embassy hangs if we use the HSE clock
        // config.rcc.hse = Some(Hse {
        //     freq: Hertz(25_000_000),
        //     mode: HseMode::Bypass,
        // });
        config.rcc.hsi = true;
        // HSI is 16MHz
        config.rcc.pll_src = PllSource::HSI;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV16,
            mul: PllMul::MUL192,
            divp: Some(PllPDiv::DIV2), // 16MHz / 16 * 192 / 2 = 96Mhz.
            divq: Some(PllQDiv::DIV4), // 16MHz / 16 * 192 / 4 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }
    let p = embassy_stm32::init(config);
    info!("clocks initialized");

    spawner.spawn(blink_on_time(p.PC13.degrade())).unwrap();

    let mut usb_buf = [0u8; 256];

    let mut config = UsbOtgConfig::default();
    config.vbus_detection = false;

    let driver = embassy_stm32::usb::Driver::new_fs(
        p.USB_OTG_FS,
        UsbIrq,
        p.PA12,
        p.PA11,
        &mut usb_buf,
        config
    );

    let mut config = UsbConfig::new(0x1209, 0x0011);
    config.manufacturer = Some("Juliapixel");
    config.product = Some("Palm USB Keyboard");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // Microsoft OS descriptor
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut handler = MyUsbHandler::new();
    let mut state = State::new();

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf
    );

    builder.handler(&mut handler);

    let mut request_handler = MyRequestHandler {};

    let config = HidConfig {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 10,
        max_packet_size: 8
    };

    let hid = HidReaderWriter::<'_, _, 1, 8>::new(&mut builder, &mut state, config);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let (mut reader, writer) = hid.split();

    let read_fut = async {
        debug!("waiting for reader");
        reader.ready().await;
        debug!("reader running");
        reader.run(false, &mut request_handler).await;
    };

    let uart_fut = async {
        let mut config = UsartConfig::default();
        config.baudrate = 9600;
        config.data_bits = DataBits::DataBits8;
        config.parity = Parity::ParityNone;
        config.stop_bits = StopBits::STOP1;

        let rxd_pin = p.PA3;
        let dma_chan = p.DMA1_CH5;

        let uart = UartRx::new(
            p.USART2,
            UsartIrq {},
            &rxd_pin,
            dma_chan,
            config
        ).unwrap();

        let driver = KeyboardDriver::new(uart, p.PB8, p.PB4, p.PB3, p.EXTI3, writer);
        driver.run().await
    };

    let failed = select3(usb_fut, read_fut, uart_fut).await;
    match failed {
        Either3::First(_) => error!("usb driver task exited unexpectedly"),
        Either3::Second(_) => error!("usb reader task exited unexpectedly"),
        Either3::Third(_) => error!("uart task exited unexpectedly"),
    }
}

#[embassy_executor::task]
async fn blink_on_time(pin: AnyPin) {
    let mut blinky = Output::new(
        pin,
        embassy_stm32::gpio::Level::Low,
        embassy_stm32::gpio::Speed::Low
    );
    loop {
        blinky.set_low();
        Timer::after(Duration::from_millis(20)).await;
        blinky.set_high();
        Timer::after(Duration::from_millis(50)).await;
        blinky.set_low();
        Timer::after(Duration::from_millis(20)).await;
        blinky.set_high();
        Timer::after(Duration::from_millis(1950)).await;
    }
}

async fn log_uart<T: Pin>(kb_vcc: AnyPin, exti: impl Peripheral<P = T::ExtiChannel>, kb_monitor: impl Peripheral<P = T>) {
    let mut vcc = Output::new(kb_vcc, embassy_stm32::gpio::Level::Low, embassy_stm32::gpio::Speed::Low);
    vcc.set_high();
    Timer::after(Duration::from_millis(500)).await;

    let mut input = ExtiInput::new(kb_monitor, exti, embassy_stm32::gpio::Pull::Down);
    loop {
        input.wait_for_any_edge().await;
        match input.get_level() {
            embassy_stm32::gpio::Level::Low => {
                debug!("UART low")
            },
            embassy_stm32::gpio::Level::High => {
                debug!("UART high")
            },
        }
    }
}
