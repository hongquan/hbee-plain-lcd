mod consts;
#[allow(dead_code)]
mod display;
mod macros;
mod thingsup;
mod types;
mod ui;

use core::time::Duration;
use std::borrow::Borrow;
use std::thread::sleep;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::reset::restart;
use log::info;
use slint::ComponentHandle;

use crate::thingsup::init_front_display;
use crate::thingsup::init_window;
use crate::types::{AppConfig, SensorDataMessage};
use crate::types::{AppError, HBeePeripherals};
use crate::ui::UILineRenderrer;

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
    info!("Setup LCD front display");
    let mut winsys = init_window()?;
    info!("Created window system.");
    let app_config = AppConfig {
        serial_number: "hb00000".to_string(),
        farm_codename: "happy-farm".to_string(),
    };

    let mut ss = SensorDataMessage::new_random();

    winsys
        .app_window
        .global::<ui::NativeUtils>()
        .on_display_float(|n| n.to_string().replace(".", ",").into());
    winsys
        .app_window
        .set_farm_codename(app_config.farm_codename.as_str().into());
    winsys
        .app_window
        .set_serial_number(app_config.serial_number.as_str().into());
    winsys.app_window.set_sensor(ss.borrow().into());
    winsys.slint_window.draw_if_needed(|renderer| {
        renderer.render_by_line(UILineRenderrer {
            display: &mut front_display,
            line_buffer: &mut winsys.line_buffer,
        });
    });
    sleep(Duration::from_secs(1));

    loop {
        ss.regen_random();
        info!("Random data: {ss:?}");
        winsys.app_window.set_sensor(ss.borrow().into());
        winsys.slint_window.draw_if_needed(|renderer| {
            renderer.render_by_line(UILineRenderrer {
                display: &mut front_display,
                line_buffer: &mut winsys.line_buffer,
            });
        });
        sleep(Duration::from_secs(5));
    }
}
