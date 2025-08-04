use crate::data::GaugeData;
use embedded_graphics::Drawable;
use embedded_graphics::prelude::{Primitive, WebColors};
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point, RgbColor, Size},
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use profont::{PROFONT_14_POINT, PROFONT_24_POINT};
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

    let text_style_primary = MonoTextStyle::new(&PROFONT_24_POINT, Rgb565::WHITE);
    let text_style_secondary = MonoTextStyle::new(&PROFONT_14_POINT, Rgb565::CSS_LIGHT_SLATE_GRAY);

    Text::new(
        &format!("{:.1}C", data.oil_temp_est),
        Point::new(75, 80),
        text_style_primary,
    )
    .draw(display)?;
    Text::new(
        &format!("oil est."),
        Point::new(80, 105),
        text_style_secondary,
    )
    .draw(display)?;

    Text::new(
        &format!("{:.1}C", data.coolant_temp),
        Point::new((w / 2 + 75) as i32, 80),
        text_style_primary,
    )
    .draw(display)?;
    Text::new(
        &format!("water"),
        Point::new((w / 2 + 87) as i32, 105),
        text_style_secondary,
    )
    .draw(display)?;

    Text::new(
        &format!("{:.1}V", data.voltage),
        Point::new(75, (h / 2 + 80) as i32),
        text_style_primary,
    )
    .draw(display)?;
    Text::new(
        &format!("voltage"),
        Point::new(77, (h / 2 + 105) as i32),
        text_style_secondary,
    )
    .draw(display)?;

    Text::new(
        &format!("{}%", data.engine_load.round()),
        Point::new((w / 2 + 90) as i32, (h / 2 + 80) as i32),
        text_style_primary,
    )
    .draw(display)?;
    Text::new(
        &format!("load"),
        Point::new((w / 2 + 93) as i32, (h / 2 + 105) as i32),
        text_style_secondary,
    )
    .draw(display)?;

    Ok(())
}
