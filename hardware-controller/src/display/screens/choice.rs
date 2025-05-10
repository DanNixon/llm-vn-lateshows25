use embedded_graphics::{
    geometry::AnchorPoint,
    mono_font::{ascii::FONT_9X18, MonoTextStyle},
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Primitive, Size, WebColors},
    primitives::PrimitiveStyleBuilder,
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

pub(crate) struct ChoiceScreen {
    content: icd::ChoiceScreen,
}

impl ChoiceScreen {
    pub(crate) fn new(content: icd::ChoiceScreen) -> Self {
        Self { content }
    }
}

impl Drawable for ChoiceScreen {
    type Color = Rgb666;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.clear(Rgb666::CSS_BLACK)?;

        for (i, rect) in super::choice_boxes(target).into_iter().enumerate() {
            rect.into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(self.content.margin_colour())
                    .fill_color(self.content.background_colour())
                    .build(),
            )
            .draw(target)?;

            TextBox::with_textbox_style(
                self.content.choice_text(i),
                rect.resized(rect.size - Size::new(8, 8), AnchorPoint::Center),
                MonoTextStyle::new(&FONT_9X18, self.content.text_colour()),
                TextBoxStyleBuilder::new()
                    .alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Top)
                    .build(),
            )
            .draw(target)?;
        }

        Ok(())
    }
}
