use crate::conversation::VnOutput;
use embedded_graphics::pixelcolor::{Rgb666, Rgb888};
use icd::{CharacterDetails, ChoiceScreen};
use log::debug;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CharacterCollection {
    pub characters: Vec<Character>,
}

impl CharacterCollection {
    pub(crate) fn load(s: &Path) -> Self {
        let content = std::fs::read_to_string(s).expect("Failed to read character file");
        let characters: Self = toml::from_str(&content).expect("Failed to parse character file");
        debug!("Loaded characters: {characters:#?}");

        assert!(
            characters.characters.len() >= 3,
            "There must be at least three characters defined."
        );

        characters
    }

    pub(crate) fn pick_subset(&self, idx: usize) -> [&Character; 3] {
        let indices = if idx == 0 {
            [self.characters.len() - 1, 0, 1]
        } else if idx == self.characters.len() - 1 {
            [idx - 1, idx, 0]
        } else {
            [idx - 1, idx, idx + 1]
        };

        [
            &self.characters[indices[0]],
            &self.characters[indices[1]],
            &self.characters[indices[2]],
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Colour> for Rgb888 {
    fn from(c: Colour) -> Self {
        Self::new(c.r, c.g, c.b)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Character {
    pub name: String,
    pub description: String,

    pub model_name: String,

    text_colour: Colour,
    background_colour: Colour,
    border_colour: Colour,

    opening_lines: Vec<String>,
}

impl Character {
    pub(crate) fn text_colour(&self) -> Rgb666 {
        let c: Rgb888 = self.text_colour.clone().into();
        c.into()
    }

    pub(crate) fn background_colour(&self) -> Rgb666 {
        let c: Rgb888 = self.background_colour.clone().into();
        c.into()
    }

    pub(crate) fn border_colour(&self) -> Rgb666 {
        let c: Rgb888 = self.border_colour.clone().into();
        c.into()
    }

    pub(crate) fn starting_phrases(&self) -> VnOutput {
        let mut rng = rand::rng();
        let lines: Vec<&String> = self.opening_lines.choose_multiple(&mut rng, 3).collect();
        VnOutput {
            response: String::default(),
            user_reply_1: lines[0].into(),
            user_reply_2: lines[1].into(),
            user_reply_3: lines[2].into(),
        }
    }

    pub(crate) fn choice_screen(&self, last: &VnOutput) -> ChoiceScreen {
        ChoiceScreen::new(
            self.text_colour(),
            self.background_colour(),
            self.border_colour(),
            last.user_reply_1.as_str().try_into().unwrap(),
            last.user_reply_2.as_str().try_into().unwrap(),
            last.user_reply_3.as_str().try_into().unwrap(),
        )
    }
}

impl From<Character> for CharacterDetails {
    fn from(value: Character) -> Self {
        Self::new(
            value.text_colour(),
            value.background_colour(),
            value.border_colour(),
            value.name.as_str().try_into().unwrap(),
            value.description.as_str().try_into().unwrap(),
        )
    }
}
