use std::rc::Rc;

use display_interface_spi::SPIInterface;
use esp_idf_svc::hal::gpio::{
    Gpio10, Gpio11, Gpio12, Gpio13, Gpio14, Gpio4, Gpio9, Output, PinDriver,
};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::spi::{SpiDeviceDriver, SpiDriver, SPI2};
use esp_idf_svc::sys::EspError;
use mipidsi::error::Error as DisplayError;
use mipidsi::models::ILI9341Rgb565;
use serde::Serialize;
use serde_with::skip_serializing_none;
use slint::platform::software_renderer::{MinimalSoftwareWindow, Rgb565Pixel};
use slint::platform::SetPlatformError;
use slint::PlatformError;
use u8g2_fonts::Error as FontError;

use crate::consts::DISPLAY_WIDTH;

pub struct FrontDisplayBlock {
    pub spi: SPI2,
    #[allow(dead_code)]
    pub pin_cs: Gpio10,
    pub pin_reset: Gpio9,
    pub pin_dc: Gpio4,
    pub pin_mosi: Gpio11,
    pub pin_clk: Gpio12,
    pub pin_miso: Gpio13,
    pub pin_backlight: Gpio14,
}

pub struct HBeePeripherals {
    pub front_display: FrontDisplayBlock,
}

impl From<Peripherals> for HBeePeripherals {
    fn from(p: Peripherals) -> Self {
        let front_display = FrontDisplayBlock {
            spi: p.spi2,
            pin_cs: p.pins.gpio10,
            pin_reset: p.pins.gpio9,
            pin_dc: p.pins.gpio4,
            pin_mosi: p.pins.gpio11,
            pin_clk: p.pins.gpio12,
            pin_miso: p.pins.gpio13,
            pin_backlight: p.pins.gpio14,
        };
        Self { front_display }
    }
}

pub type FrontDisplayDriver<'d> = mipidsi::Display<
    SPIInterface<SpiDeviceDriver<'d, SpiDriver<'d>>, PinDriver<'d, Gpio4, Output>>,
    ILI9341Rgb565,
    PinDriver<'d, Gpio9, Output>,
>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    EspError(#[from] EspError),
    #[error("Display error: {0:?}")]
    DisplayError(DisplayError),
    #[error("{0:?}")]
    FontError(FontError<DisplayError>),
    #[error("{0:?}")]
    PlatformError(PlatformError),
    #[error("{0:?}")]
    SetPlatformError(SetPlatformError),
}

impl From<DisplayError> for AppError {
    fn from(e: DisplayError) -> Self {
        Self::DisplayError(e)
    }
}

impl From<FontError<DisplayError>> for AppError {
    fn from(e: FontError<DisplayError>) -> Self {
        Self::FontError(e)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeviceCondition {
    pub gsm_signal_strength: Option<i16>,
    pub battery_level: Option<u8>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
pub struct SensorDataMessage {
    #[serde(rename = "solEC")]
    pub electrical_conductivity: Option<u16>,
    #[serde(rename = "solRes")]
    pub electrical_resistivity: Option<f32>,
    #[serde(rename = "solPH")]
    pub power_of_hydrogen: Option<f32>,
    #[serde(rename = "solT")]
    pub solution_temperature: Option<f32>,
    #[serde(rename = "solTDS")]
    pub total_dissolved_solids: Option<f32>,
    #[serde(rename = "solSal")]
    pub solution_salinity: Option<f32>,
}

impl SensorDataMessage {
    pub fn new_random() -> Self {
        let electrical_conductivity = Some(fastrand::u16(1000..2000));
        let solution_salinity = Some(fastrand::u16(1000..2000) as f32);
        let power_of_hydrogen = Some(fastrand::u8(40..80) as f32 / 10.0);
        let solution_temperature = Some(fastrand::u16(200..300) as f32 / 10.0);
        Self {
            electrical_conductivity,
            solution_salinity,
            power_of_hydrogen,
            solution_temperature,
            ..Default::default()
        }
    }

    pub fn regen_random(&mut self) {
        self.electrical_conductivity = Some(fastrand::u16(1000..2000));
        self.solution_salinity = Some(fastrand::u16(1000..2000) as f32);
        self.power_of_hydrogen = Some(fastrand::u8(40..80) as f32 / 10.0);
        self.solution_temperature = Some(fastrand::u16(200..300) as f32 / 10.0);
    }
}

#[allow(dead_code)]
pub(crate) struct WindowSystem {
    pub(crate) slint_window: Rc<MinimalSoftwareWindow>,
    pub(crate) app_window: crate::ui::AppWindow,
    pub(crate) line_buffer: [Rgb565Pixel; DISPLAY_WIDTH as usize],
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub serial_number: String,
    pub farm_codename: String,
}

#[derive(Debug)]
pub enum UIInitError {
    PlatformError(PlatformError),
    SetPlatformError(SetPlatformError),
}

impl From<PlatformError> for UIInitError {
    fn from(e: PlatformError) -> Self {
        Self::PlatformError(e)
    }
}

impl From<SetPlatformError> for UIInitError {
    fn from(e: SetPlatformError) -> Self {
        Self::SetPlatformError(e)
    }
}

impl From<UIInitError> for AppError {
    fn from(value: UIInitError) -> Self {
        match value {
            UIInitError::PlatformError(e) => Self::PlatformError(e),
            UIInitError::SetPlatformError(e) => Self::SetPlatformError(e),
        }
    }
}
