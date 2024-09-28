mod consts;
mod display;
mod macros;
mod thingsup;
mod types;

use core::time::Duration;
use std::thread::sleep;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::reset::restart;

use crate::display::show_intro;
use crate::thingsup::init_front_display;
use crate::types::{AppError, HBeePeripherals};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    if let Err(e) = try_serving() {
        let d = Duration::from_secs(10);
        log::error!("Got error: {}. To restart in {}s...", e, d.as_secs());
        sleep(d);
        restart();
    }
}

fn try_serving() -> Result<(), AppError> {
    let peripherals = Peripherals::take()?;
    let periph = HBeePeripherals::from(peripherals);
    let mut front_display = init_front_display(periph.front_display)?;
    show_intro(&mut front_display)?;
    loop {
        sleep(Duration::from_secs(5));
    }
}
