use crate::data::GaugeData;
use embedded_graphics::Drawable;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor, Size},
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use profont::PROFONT_24_POINT;
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

    let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb565::WHITE);

    Text::new(
        &format!("{} HP", data.horse_power),
        Point::new(75, 80),
        text_style,
    )
    .draw(display)?;
    Text::new(
        &format!("{:.1}°C", data.coolant_temp),
        Point::new((w / 2 + 75) as i32, 80),
        text_style,
    )
    .draw(display)?;
    Text::new(
        &format!("{:.1}V", data.voltage),
        Point::new(75, (h / 2 + 80) as i32),
        text_style,
    )
    .draw(display)?;
    Text::new(
        &format!("{}%", data.engine_load),
        Point::new((w / 2 + 90) as i32, (h / 2 + 80) as i32),
        text_style,
    )
    .draw(display)?;

    Ok(())
}
