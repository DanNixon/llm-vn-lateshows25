#![cfg_attr(not(feature = "use-std"), no_std)]

use embedded_graphics::{
    pixelcolor::{raw::RawU24, Rgb666},
    prelude::RawData,
};
use postcard_rpc::{endpoints, topics, TopicDirection};
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

endpoints! {
    list = ENDPOINT_LIST;
    omit_std = true;
    | EndpointTy   | RequestTy | ResponseTy | Path          |
    | ----------   | --------- | ---------- | ----          |
    | SetDisplay   | Screen    | ()         | "set_display" |
}

topics! {
    list = TOPICS_IN_LIST;
    direction = TopicDirection::ToServer;
    | TopicTy | MessageTy | Path |
    | ------- | --------- | ---- |
}

topics! {
    list = TOPICS_OUT_LIST;
    direction = TopicDirection::ToClient;
    | TopicTy               | MessageTy    | Path            | Cfg |
    | -------               | ---------    | ----            | --- |
    | ButtonActionPerformed | ButtonAction | "button_action" |     |
}

#[derive(Debug, defmt::Format, Clone, Serialize, Deserialize, Schema)]
#[allow(clippy::large_enum_variant)]
pub enum Screen {
    CharacterSelect(CharacterSelectScreen),
    Choices(ChoiceScreen),
}

#[derive(Debug, defmt::Format, Clone, Serialize, Deserialize, Schema)]
pub struct CharacterDetails {
    text_colour: u32,
    background_colour: u32,
    margin_colour: u32,
    pub name: heapless::String<32>,
    pub description: heapless::String<512>,
}

impl CharacterDetails {
    pub fn new(
        text_colour: Rgb666,
        background_colour: Rgb666,
        margin_colour: Rgb666,
        name: heapless::String<32>,
        description: heapless::String<512>,
    ) -> Self {
        Self {
            text_colour: rgb666_to_u32(text_colour),
            background_colour: rgb666_to_u32(background_colour),
            margin_colour: rgb666_to_u32(margin_colour),
            name,
            description,
        }
    }

    pub fn text_colour(&self) -> Rgb666 {
        rgb666_from_u32(self.text_colour)
    }

    pub fn background_colour(&self) -> Rgb666 {
        rgb666_from_u32(self.background_colour)
    }

    pub fn margin_colour(&self) -> Rgb666 {
        rgb666_from_u32(self.margin_colour)
    }
}

#[derive(Debug, defmt::Format, Clone, Serialize, Deserialize, Schema)]
pub struct CharacterSelectScreen {
    pub prev: CharacterDetails,
    pub selected: CharacterDetails,
    pub next: CharacterDetails,
}

pub type ChoiceString = heapless::String<256>;

#[derive(Debug, defmt::Format, Clone, Serialize, Deserialize, Schema)]
pub struct ChoiceScreen {
    text_colour: u32,
    background_colour: u32,
    margin_colour: u32,

    choice_1: ChoiceString,
    choice_2: ChoiceString,
    choice_3: ChoiceString,
}

impl ChoiceScreen {
    pub fn new(
        text_colour: Rgb666,
        background_colour: Rgb666,
        margin_colour: Rgb666,
        choice_1: ChoiceString,
        choice_2: ChoiceString,
        choice_3: ChoiceString,
    ) -> Self {
        Self {
            text_colour: rgb666_to_u32(text_colour),
            background_colour: rgb666_to_u32(background_colour),
            margin_colour: rgb666_to_u32(margin_colour),
            choice_1,
            choice_2,
            choice_3,
        }
    }

    pub fn text_colour(&self) -> Rgb666 {
        rgb666_from_u32(self.text_colour)
    }

    pub fn background_colour(&self) -> Rgb666 {
        rgb666_from_u32(self.background_colour)
    }

    pub fn margin_colour(&self) -> Rgb666 {
        rgb666_from_u32(self.margin_colour)
    }

    pub fn choice_text(&self, idx: usize) -> &str {
        match idx {
            0 => &self.choice_1,
            1 => &self.choice_2,
            2 => &self.choice_3,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, defmt::Format, Clone, Serialize, Deserialize, Schema)]
pub enum ButtonAction {
    Fn1,
    Fn2,
    Fn3,
    EndConversation,
}

fn rgb666_to_u32(c: Rgb666) -> u32 {
    let c: RawU24 = c.into();
    c.into_inner()
}

fn rgb666_from_u32(c: u32) -> Rgb666 {
    let c = RawU24::new(c);
    Rgb666::from(c)
}
