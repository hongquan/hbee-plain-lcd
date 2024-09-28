use core::time::Duration;
use std::rc::Rc;

use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use esp_idf_svc::systime::EspSystemTime;
use log::error;
use slint::platform::software_renderer::{LineBufferProvider, MinimalSoftwareWindow, Rgb565Pixel};
use slint::platform::{Platform, WindowAdapter};

use crate::types::DeviceCondition;
use crate::types::SensorDataMessage;

slint::include_modules!();

impl From<&SensorDataMessage> for SensorData {
    fn from(v: &SensorDataMessage) -> Self {
        Self {
            solution_temperature: v.solution_temperature.unwrap_or_default(),
            has_solution_temperature: v.solution_temperature.is_some(),
            power_of_hydrogen: v.power_of_hydrogen.unwrap_or_default(),
            has_power_of_hydrogen: v.power_of_hydrogen.is_some(),
            electrical_conductivity: v.electrical_conductivity.unwrap_or_default().into(),
            has_electrical_conductivity: v.electrical_conductivity.is_some(),
            electrical_resistivity: v.electrical_resistivity.unwrap_or_default(),
            has_electrical_resistivity: v.electrical_resistivity.is_some(),
            solution_salinity: v.solution_salinity.unwrap_or_default(),
            has_solution_salinity: v.solution_salinity.is_some(),
            total_dissolved_solids: v.total_dissolved_solids.unwrap_or_default(),
            has_total_dissolved_solids: v.total_dissolved_solids.is_some(),
        }
    }
}

impl From<&DeviceCondition> for Health {
    fn from(v: &DeviceCondition) -> Self {
        Self {
            gsm_signal_strength: v.gsm_signal_strength.unwrap_or_default().into(),
            has_gsm_signal_strength: v.gsm_signal_strength.is_some(),
            battery_level: v.battery_level.unwrap_or_default().into(),
            has_battery_level: v.battery_level.is_some(),
        }
    }
}

pub struct UILineRenderrer<'a, T> {
    pub display: &'a mut T,
    pub line_buffer: &'a mut [Rgb565Pixel],
}

impl<T: DrawTarget<Color = Rgb565>> LineBufferProvider for UILineRenderrer<'_, T> {
    type TargetPixel = Rgb565Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        // Render into the line
        render_fn(&mut self.line_buffer[range.clone()]);

        // Send the line to the screen using DrawTarget::fill_contiguous
        let colors = self.line_buffer[range.clone()]
            .iter()
            .map(|p| RawU16::new(p.0).into());
        let area = Rectangle::new(
            Point::new(range.start as _, line as _),
            Size::new(range.len() as _, 1),
        );
        self.display
            .fill_contiguous(&area, colors)
            .unwrap_or_else(|_e| error!("Error when drawing to LCD!"));
    }
}

pub struct UIPlatform(pub Rc<MinimalSoftwareWindow>);

impl Platform for UIPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        Ok(self.0.clone())
    }

    fn duration_since_start(&self) -> Duration {
        EspSystemTime.now()
    }
}
