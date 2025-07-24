mod config;
mod data;
mod display;
mod obd;

use embedded_graphics::prelude::*;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, RgbColor, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

use crate::{data::GaugeData, display::layout::draw_ui, obd::ObdReader};

fn main() {
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("RX-8 Gauge Simulator", &output_settings);
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(480, 320));
    let mut data_source = obd::init_obd();

    'running: loop {
        window.update(&display);

        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'running;
            }
        }

        let data: GaugeData = data_source.read_data();

        let black_fill = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        Rectangle::new(Point::zero(), display.bounding_box().size)
            .into_styled(black_fill)
            .draw(&mut display)
            .unwrap();

        let _ = draw_ui(&mut display, &data);

        thread::sleep(Duration::from_millis(200));
    }
}
