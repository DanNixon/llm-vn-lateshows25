mod character_select;
mod choice;
mod splash;

use embedded_graphics::{
    geometry::AnchorPoint,
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};
use heapless::Vec;

pub(crate) use self::{
    character_select::CharacterSelectScreen, choice::ChoiceScreen, splash::SplashScreen,
};

fn choice_boxes<D>(target: &D) -> Vec<Rectangle, 3>
where
    D: DrawTarget<Color = Rgb666>,
{
    let screen_box = target.bounding_box();
    let option_height = screen_box.size.height / 3;

    (0..3)
        .map(|i| {
            let size = Size::new(screen_box.size.width, option_height);
            let inner_size = size - Size::new(0, 4);

            Rectangle::new(Point::new(0, (option_height * i) as i32), size)
                .resized(inner_size, AnchorPoint::Center)
        })
        .collect()
}
