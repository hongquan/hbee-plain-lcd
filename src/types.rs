use display_interface_spi::SPIInterface;
use esp_idf_svc::hal::gpio::{Gpio10, Gpio11, Gpio12, Gpio13, Gpio46, Gpio9, Output, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::spi::{SpiDeviceDriver, SpiDriver, SPI2};
use esp_idf_svc::sys::EspError;
use mipidsi::error::Error as DisplayError;
use mipidsi::models::ILI9341Rgb565;
use u8g2_fonts::Error as FontError;

pub struct FrontDisplayBlock {
    pub spi: SPI2,
    pub pin_cs: Gpio10,
    pub pin_reset: Gpio9,
    pub pin_dc: Gpio46,
    pub pin_mosi: Gpio11,
    pub pin_clk: Gpio12,
    pub pin_miso: Gpio13,
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
            pin_dc: p.pins.gpio46,
            pin_mosi: p.pins.gpio11,
            pin_clk: p.pins.gpio12,
            pin_miso: p.pins.gpio13,
        };
        Self { front_display }
    }
}

pub type FrontDisplayDriver<'d> = mipidsi::Display<
    SPIInterface<SpiDeviceDriver<'d, SpiDriver<'d>>, PinDriver<'d, Gpio46, Output>>,
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
