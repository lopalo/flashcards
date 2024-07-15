use implicit_clone::ImplicitClone;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use ulid::Ulid;
use yew::Reducible;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug,
)]
pub enum Language {
    English,
    German,
    Spanish,
    Polish,
    Slovak,
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
        &[English, German, Spanish, Polish, Slovak, Ukranian]
    }
}

impl ImplicitClone for Language {}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct FlashcardSide {
    pub text: String,
    pub language: Language,
}

impl FlashcardSide {
    pub fn words(&self) -> Vec<&str> {
        self.text.as_str().split_whitespace().collect()
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: String,
    pub front_side: FlashcardSide,
    pub back_side: FlashcardSide,
}

impl Flashcard {
    pub fn new() -> Self {
        let side = FlashcardSide {
            text: Default::default(),
            language: Language::English,
        };
        Self {
            id: Ulid::new().to_string(),
            front_side: side.clone(),
            back_side: side,
        }
    }
}

pub enum FlashcardAction {
    ReplaceWithNew,
    ReplaceWith(Rc<Flashcard>),
    SetFrontText(String),
    SetBackText(String),
    SetFrontLanguage(Language),
    SetBackLanguage(Language),
}

impl Reducible for Flashcard {
    type Action = FlashcardAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use FlashcardAction::*;
        let this = Rc::make_mut(&mut self);

        match action {
            ReplaceWithNew => *this = Self::new(),
            ReplaceWith(flashcard) => {
                *this = (*flashcard).clone()
            }
            SetFrontText(text) => this.front_side.text = text,
            SetBackText(text) => this.back_side.text = text,
            SetFrontLanguage(language) => this.front_side.language = language,
            SetBackLanguage(language) => this.back_side.language = language,
        };

        self
    }
}
