use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum Language {
    English,
    Ukranian,
    Polish,
}

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
