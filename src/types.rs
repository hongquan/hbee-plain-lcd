use esp_idf_svc::hal::gpio::{Gpio10, Gpio11, Gpio12, Gpio13, Gpio46, Gpio9};
use esp_idf_svc::hal::spi::SPI2;

pub struct FrontDisplayBlock {
    pub spi: SPI2,
    pub pin_cs: Gpio10,
    pub pin_reset: Gpio9,
    pub pin_dc: Gpio46,
    pub pin_mosi: Gpio11,
    pub pin_clk: Gpio12,
    pub pin_miso: Gpio13,
}
