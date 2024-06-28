use super::flashcard::Language;
use crate::local_storage::LocalStorageRecord;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, rc::Rc};
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub repeat_card_after_n_cards: usize,
    pub voices: BTreeMap<Language, String>,
}

impl Settings {
    pub fn get_voice(&self, lang: Language) -> &str {
        self.voices
            .get(&lang)
            .map(AsRef::as_ref)
            .unwrap_or("Google UK English Male")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            repeat_card_after_n_cards: 2,
            voices: Default::default(),
        }
    }
}

impl Reducible for Settings {
    type Action = ();

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        self
    }
}

impl LocalStorageRecord for Settings {
    const KEY: &'static str = "settings";
}
