use core::sync::atomic::{AtomicBool, Ordering};

use kb_driver_proc_macro::debug;
use embassy_usb::{class::hid::RequestHandler, Handler};

#[derive(Default)]
pub struct MyUsbHandler {
    configured: AtomicBool
}

impl MyUsbHandler {
    pub fn new() -> Self {
        Self { configured: false.into() }
    }
}

impl Handler for MyUsbHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(enabled, Ordering::Relaxed);
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
    }

    fn addressed(&mut self, _addr: u8) {}

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            debug!("device configured!");
        } else {
            debug!("device deconfigured");
        }
    }

    fn suspended(&mut self, _suspended: bool) {}

    fn remote_wakeup_enabled(&mut self, _enabled: bool) {}

    fn set_alternate_setting(&mut self, iface: embassy_usb::types::InterfaceNumber, alternate_setting: u8) {
        let _ = iface;
        let _ = alternate_setting;
    }

    fn control_out(&mut self, req: embassy_usb::control::Request, data: &[u8]) -> Option<embassy_usb::control::OutResponse> {
        let _ = (req, data);
        None
    }

    fn control_in<'a>(&'a mut self, req: embassy_usb::control::Request, buf: &'a mut [u8]) -> Option<embassy_usb::control::InResponse<'a>> {
        let _ = (req, buf);
        None
    }

    fn get_string(&mut self, index: embassy_usb::types::StringIndex, _lang_id: u16) -> Option<&str> {
        let _ = (index, embassy_usb::descriptor::lang_id::ENGLISH_US);
        None
    }
}

pub struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: embassy_usb::class::hid::ReportId, buf: &mut [u8]) -> Option<usize> {
        #[cfg(feature = "defmt")]
        debug!("received report {:?}, data {:?}", id, buf);
        let _ = (id, buf);
        None
    }

    fn set_report(&mut self, id: embassy_usb::class::hid::ReportId, data: &[u8]) -> embassy_usb::control::OutResponse {
        #[cfg(feature = "defmt")]
        debug!("received report {:?}, data: {:?}", id, data);
        embassy_usb::control::OutResponse::Accepted
    }

    fn get_idle_ms(&mut self, id: Option<embassy_usb::class::hid::ReportId>) -> Option<u32> {
        #[cfg(feature = "defmt")]
        debug!("received report {:?}", id);
        let _ = id;
        None
    }

    fn set_idle_ms(&mut self, id: Option<embassy_usb::class::hid::ReportId>, duration_ms: u32) {
        #[cfg(feature = "defmt")]
        debug!("received report {:?}, duration {}ms", id, duration_ms);
        let _ = (id, duration_ms);
    }
}
