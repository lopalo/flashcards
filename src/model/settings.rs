use super::flashcard::Language;
use crate::local_storage::LocalStorageRecord;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, rc::Rc};
use yew::prelude::*;

pub static DEFAULT_VOICE: &str = "Google UK English Male";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub repeat_card_distance: usize,
    pub voices: BTreeMap<Language, String>,
    pub default_card_front_side_language: Language,
    pub default_card_back_side_language: Language,
    pub always_speak_back_side_text: bool,
}

impl Settings {
    pub fn get_voice(&self, lang: Language) -> &str {
        self.voices
            .get(&lang)
            .map(AsRef::as_ref)
            .unwrap_or(DEFAULT_VOICE)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            repeat_card_distance: 10,
            voices: Default::default(),
            default_card_front_side_language: Language::Ukranian,
            default_card_back_side_language: Language::English,
            always_speak_back_side_text: true,
        }
    }
}

pub enum SettingsAction {
    RepeatCardDistance(usize),
    Voice { language: Language, voice: String },
    CardFrontSideLanguage(Language),
    CardBackSideLanguage(Language),
    AlwaysSpeakBackSideText(bool),
}

impl Reducible for Settings {
    type Action = SettingsAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use SettingsAction::*;
        let this = Rc::make_mut(&mut self);

        match action {
            RepeatCardDistance(distance) => {
                this.repeat_card_distance = distance
            }
            Voice { language, voice } => {
                this.voices.insert(language, voice);
            }
            CardFrontSideLanguage(language) => {
                this.default_card_front_side_language = language
            }
            CardBackSideLanguage(language) => {
                this.default_card_back_side_language = language
            }
            AlwaysSpeakBackSideText(val) => {
                this.always_speak_back_side_text = val
            }
        }

        self.save_in_local_storage();
        self
    }
}

impl LocalStorageRecord for Settings {
    const KEY: &'static str = "settings";
}
