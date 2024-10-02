use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use log::info;
use mipidsi::error::Error as DisplayError;
use u8g2_fonts::fonts::u8g2_font_unifont_t_vietnamese1;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use u8g2_fonts::Error;
use u8g2_fonts::FontRenderer;

use crate::types::FrontDisplayDriver;

pub(crate) const VI_FONT: FontRenderer = FontRenderer::new::<u8g2_font_unifont_t_vietnamese1>();

pub(crate) fn show_intro(driver: &mut FrontDisplayDriver) -> Result<(), Error<DisplayError>> {
    driver
        .clear(Rgb565::BLUE)
        .map_err(|e| Error::DisplayError(e))?;
    info!("Made background gray");
    let lines = vec![
        String::from("HBee"),
        format!("Số sê-ri: {}", "hb0000"),
        format!("Trang trại: {}", "Hạnh Phúc"),
    ];
    let content = lines.join("\n");
    VI_FONT.render_aligned(
        content.as_str(),
        Point::new(10, 10),
        VerticalPosition::Baseline,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::CSS_VIOLET),
        driver,
    )?;
    info!("Shown {} to LCD", content);
    Ok(())
}
