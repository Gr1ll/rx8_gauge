mod config;
mod data;
mod display;
mod obd;
mod platform;

#[cfg(feature = "pi")]
use std::thread;
#[cfg(feature = "pi")]
use std::time::Duration;

#[cfg(feature = "pi")]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(not(feature = "pi"))]
use embedded_graphics::pixelcolor::Rgb565;

#[cfg(feature = "pi")]
use embedded_graphics::prelude::*;
#[cfg(not(feature = "pi"))]
use embedded_graphics::prelude::*;

#[cfg(feature = "pi")]
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
#[cfg(not(feature = "pi"))]
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

#[cfg(feature = "pi")]
use crate::{data::GaugeData, display::layout::draw_ui, obd::ObdReader};
#[cfg(not(feature = "pi"))]
use crate::{data::GaugeData, display::layout::draw_ui, obd::ObdReader};

use platform::init_display;

fn main() {
    let mut display = init_display();
    let mut data_source = obd::init_obd();

    #[cfg(feature = "pi")]
    {
        loop {
            let data: GaugeData = data_source.read_data();

            let black_fill = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();

            Rectangle::new(Point::zero(), display.bounding_box().size)
                .into_styled(black_fill)
                .draw(&mut display)
                .unwrap();

            let _ = draw_ui(&mut display, &data);

            display.flush();

            thread::sleep(Duration::from_millis(100));
        }
    }

    #[cfg(not(feature = "pi"))]
    {
        use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorEvent, Window};
        use std::thread;
        use std::time::Duration;

        let output_settings = OutputSettingsBuilder::new().build();
        let mut window = Window::new("RX-8 Gauge Simulator", &output_settings);

        'running: loop {
            window.update(display.inner());

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

            thread::sleep(Duration::from_millis(100));
        }
    }
}
