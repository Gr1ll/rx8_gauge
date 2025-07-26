use embedded_graphics::Pixel;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::SimulatorDisplay;

pub struct SimDisplay {
    display: SimulatorDisplay<Rgb565>,
}

impl SimDisplay {
    pub fn new() -> Self {
        let display = SimulatorDisplay::new(Size::new(480, 320));
        Self { display }
    }

    pub fn inner(&self) -> &SimulatorDisplay<Rgb565> {
        &self.display
    }
}

impl DrawTarget for SimDisplay {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb565>>,
    {
        self.display.draw_iter(pixels)
    }
}

impl OriginDimensions for SimDisplay {
    fn size(&self) -> Size {
        self.display.size()
    }
}
