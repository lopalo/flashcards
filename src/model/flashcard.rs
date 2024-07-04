use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug,
)]
pub enum Language {
    English,
    German,
    Polish,
    Slovakian,
    Ukranian,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Language {
    pub fn all_languages() -> &'static [Self] {
        use Language::*;
        &[English, German, Polish, Slovakian, Ukranian]
    }
}

impl ImplicitClone for Language {}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct FlashcardSide {
    pub text: String,
    pub language: Language,
}

impl FlashcardSide {
    pub fn words(&self) -> Vec<&str> {
        self.text.as_str().split_whitespace().collect()
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: String,
    pub front_side: FlashcardSide,
    pub back_side: FlashcardSide,
}
