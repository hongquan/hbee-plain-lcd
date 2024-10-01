use display_interface_spi::SPIInterface;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::{AnyOutputPin, PinDriver};
use esp_idf_svc::hal::spi::{SpiConfig, SpiDeviceDriver, SpiDriverConfig};
use esp_idf_svc::hal::units::MegaHertz;
use esp_idf_svc::sys::{EspError, ESP_ERR_INVALID_STATE};
use log::{info, warn};
use mipidsi::error::InitError;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::options::{Orientation, Rotation};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel};
use slint::PhysicalSize;

use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::esp_err;
use crate::types::{FrontDisplayBlock, FrontDisplayDriver, UIInitError, WindowSystem};
use crate::ui::{self, UIPlatform};

// Ref: https://github.com/esp-rs/esp-idf-hal/blob/master/examples/spi_st7789.rs
pub(crate) fn init_front_display<'d>(
    p: FrontDisplayBlock,
) -> Result<FrontDisplayDriver<'d>, EspError> {
    let pin_rst = PinDriver::output(p.pin_reset)?;
    let pin_dc = PinDriver::output(p.pin_dc)?;
    let mut backlight = PinDriver::output(p.pin_backlight)?;
    info!("Turning on backlight for LCD");
    backlight.set_high()?;

    let config = SpiConfig::default()
        .baudrate(MegaHertz(16).into())
        .data_mode(ili9341::SPI_MODE);
    let device = SpiDeviceDriver::new_single(
        p.spi,
        p.pin_clk,
        p.pin_mosi,
        Some(p.pin_miso),
        None::<AnyOutputPin>,
        &SpiDriverConfig::default(),
        &config,
    )?;
    let spi_interface = SPIInterface::new(device, pin_dc);
    mipidsi::Builder::new(ILI9341Rgb565, spi_interface)
        .reset_pin(pin_rst)
        .display_size(DISPLAY_HEIGHT, DISPLAY_WIDTH)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut Ets)
        .map_err(|e| match e {
            InitError::Pin(v) => v.cause(),
            InitError::DisplayError => {
                warn!("Some error when setting up LCD!");
                esp_err!(ESP_ERR_INVALID_STATE)
            }
        })
}

pub(crate) fn init_window() -> Result<WindowSystem, UIInitError> {
    let slint_window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
    slint_window.set_size(PhysicalSize::new(
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
    ));
    slint::platform::set_platform(Box::new(UIPlatform(slint_window.clone())))?;
    let app_window = ui::AppWindow::new()?;
    let line_buffer = [Rgb565Pixel(0); DISPLAY_WIDTH as usize];
    Ok(WindowSystem {
        slint_window,
        app_window,
        line_buffer,
    })
}
