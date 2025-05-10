use embedded_graphics::{
    geometry::AnchorPoint,
    image::Image,
    mono_font::{
        ascii::{FONT_10X20, FONT_9X18},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Point, Primitive, Size, WebColors},
    primitives::PrimitiveStyleBuilder,
    text::{Alignment, Text},
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};
use tinybmp::Bmp;

pub(crate) struct CharacterSelectScreen {
    content: icd::CharacterSelectScreen,
}

impl CharacterSelectScreen {
    pub(crate) fn new(content: icd::CharacterSelectScreen) -> Self {
        Self { content }
    }
}

impl Drawable for CharacterSelectScreen {
    type Color = Rgb666;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.clear(Rgb666::CSS_BLACK)?;

        // Heading
        Text::with_alignment(
            "Select Character",
            target
                .bounding_box()
                .resized(
                    Size::new(target.bounding_box().size.width, 36),
                    AnchorPoint::TopCenter,
                )
                .center(),
            MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(Rgb666::CSS_WHITE)
                .underline()
                .build(),
            Alignment::Center,
        )
        .draw(target)?;

        let boxes = super::choice_boxes(target);

        let name_textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        let description_textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        // Previous function key indicator
        {
            let rect = boxes[0].resized(Size::new(32, 32), AnchorPoint::CenterLeft);
            let icon: Bmp<Rgb666> =
                Bmp::from_slice(include_bytes!("./chevron-up-solid.bmp")).unwrap();
            let image = Image::new(&icon, rect.top_left + Point::new(4, 0));
            image.draw(target)?;
        }

        // Next function key indicator
        {
            let rect = boxes[2].resized(Size::new(32, 32), AnchorPoint::CenterLeft);
            let icon: Bmp<Rgb666> =
                Bmp::from_slice(include_bytes!("./chevron-down-solid.bmp")).unwrap();
            let image = Image::new(&icon, rect.top_left + Point::new(4, 0));
            image.draw(target)?;
        }

        // Previous character
        {
            let rect = boxes[0].resized(Size::new(160, 28), AnchorPoint::BottomCenter);

            rect.into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(self.content.prev.margin_colour())
                    .fill_color(self.content.prev.background_colour())
                    .build(),
            )
            .draw(target)?;

            TextBox::with_textbox_style(
                &self.content.prev.name,
                rect,
                MonoTextStyle::new(&FONT_10X20, self.content.prev.text_colour()),
                name_textbox_style,
            )
            .draw(target)?;
        }

        // Selected character
        {
            let rect = boxes[1];

            rect.into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(self.content.selected.margin_colour())
                    .fill_color(self.content.selected.background_colour())
                    .build(),
            )
            .draw(target)?;

            TextBox::with_textbox_style(
                &self.content.selected.name,
                rect.resized(Size::new(rect.size.width, 24), AnchorPoint::TopCenter),
                MonoTextStyle::new(&FONT_10X20, self.content.selected.text_colour()),
                name_textbox_style,
            )
            .draw(target)?;

            TextBox::with_textbox_style(
                &self.content.selected.description,
                rect.resized(
                    Size::new(rect.size.width - 8, rect.size.height - 24),
                    AnchorPoint::BottomCenter,
                ),
                MonoTextStyle::new(&FONT_9X18, self.content.selected.text_colour()),
                description_textbox_style,
            )
            .draw(target)?;
        }

        // Next character
        {
            let rect = boxes[2].resized(Size::new(160, 28), AnchorPoint::TopCenter);

            rect.into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(self.content.next.margin_colour())
                    .fill_color(self.content.next.background_colour())
                    .build(),
            )
            .draw(target)?;

            TextBox::with_textbox_style(
                &self.content.next.name,
                rect,
                MonoTextStyle::new(&FONT_10X20, self.content.next.text_colour()),
                name_textbox_style,
            )
            .draw(target)?;
        }

        Ok(())
    }
}
