use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Primitive, WebColors},
    primitives::PrimitiveStyleBuilder,
    text::{Alignment, Text},
    Drawable,
};

pub(crate) struct SplashScreen {}

impl Drawable for SplashScreen {
    type Color = Rgb666;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.clear(Rgb666::CSS_HOT_PINK)?;

        Text::with_alignment(
            "llm-vn-controller",
            target.bounding_box().center(),
            MonoTextStyle::new(&FONT_10X20, Rgb666::CSS_BLACK),
            Alignment::Center,
        )
        .draw(target)?;

        target
            .bounding_box()
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(Rgb666::CSS_CYAN)
                    .build(),
            )
            .draw(target)?;

        Ok(())
    }
}
