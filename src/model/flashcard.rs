#[derive(PartialEq)]
pub enum Language {
    Engish,
    Ukranian,
    #[allow(unused)]
    Polish,
}

#[derive(PartialEq)]
pub struct FlashcardSide {
    pub text: String,
    pub language: Language,
}

#[derive(PartialEq)]
pub struct Flashcard {
    pub id: String,
    pub front_side: FlashcardSide,
    pub back_side: FlashcardSide,
}
