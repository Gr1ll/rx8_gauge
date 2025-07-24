use crate::data::GaugeData;
use embedded_graphics::Drawable;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::{
    mono_font::{MonoTextStyle, jis_x0201::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor, Size},
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
pub fn draw_ui<D: DrawTarget<Color = Rgb565>>(
    display: &mut D,
    data: &GaugeData,
) -> Result<(), D::Error> {
    let style = PrimitiveStyle::with_stroke(Rgb565::WHITE, 1);
    let size = display.bounding_box().size;
    let (w, h) = (size.width, size.height);

    Rectangle::new(Point::zero(), Size::new(w / 2, h / 2))
        .into_styled(style)
        .draw(display)?;
    Rectangle::new(Point::new(w as i32 / 2, 0), Size::new(w / 2, h / 2))
        .into_styled(style)
        .draw(display)?;
    Rectangle::new(Point::new(0, h as i32 / 2), Size::new(w / 2, h / 2))
        .into_styled(style)
        .draw(display)?;
    Rectangle::new(
        Point::new(w as i32 / 2, h as i32 / 2),
        Size::new(w / 2, h / 2),
    )
    .into_styled(style)
    .draw(display)?;

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);

    Text::new(
        &format!("RPM: {}", data.rpm),
        Point::new(10, 20),
        text_style,
    )
    .draw(display)?;
    Text::new(
        &format!("Coolant: {:.1}°C", data.coolant_temp),
        Point::new((w / 2 + 10) as i32, 20),
        text_style,
    )
    .draw(display)?;
    Text::new(
        &format!("Voltage: {:.1}V", data.voltage),
        Point::new(10, (h / 2 + 20) as i32),
        text_style,
    )
    .draw(display)?;
    Text::new(
        &format!("Speed: {} km/h", data.speed),
        Point::new((w / 2 + 10) as i32, (h / 2 + 20) as i32),
        text_style,
    )
    .draw(display)?;

    Ok(())
}
