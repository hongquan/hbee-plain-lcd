use display_interface_spi::SPIInterface;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::spi::config::MODE_3;
use esp_idf_svc::hal::spi::{SpiConfig, SpiDeviceDriver, SpiDriverConfig};
use esp_idf_svc::hal::units::MegaHertz;
use esp_idf_svc::sys::{EspError, ESP_ERR_INVALID_STATE};
use log::warn;
use mipidsi::error::InitError;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::options::{Orientation, Rotation};

use crate::consts::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::esp_err;
use crate::types::{FrontDisplayBlock, FrontDisplayDriver};

pub(crate) fn init_front_display<'d>(
    p: FrontDisplayBlock,
) -> Result<FrontDisplayDriver<'d>, EspError> {
    let rst = PinDriver::output(p.pin_reset)?;
    let dc = PinDriver::output(p.pin_dc)?;

    let config = SpiConfig::default()
        .baudrate(MegaHertz(26).into())
        .data_mode(MODE_3);
    let device = SpiDeviceDriver::new_single(
        p.spi,
        p.pin_clk,
        p.pin_mosi,
        Some(p.pin_miso),
        Some(p.pin_cs),
        &SpiDriverConfig::default(),
        &config,
    )?;
    let spi_interface = SPIInterface::new(device, dc);
    mipidsi::Builder::new(ILI9341Rgb565, spi_interface)
        .reset_pin(rst)
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
