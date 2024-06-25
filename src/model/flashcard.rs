use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
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

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: String,
    pub front_side: FlashcardSide,
    pub back_side: FlashcardSide,
}
