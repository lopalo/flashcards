use super::flashcard::{Flashcard, FlashcardSide, Language};
use std::{collections::VecDeque, rc::Rc};

pub type LearningSet = VecDeque<Rc<Flashcard>>;

pub fn test_flashcards() -> LearningSet {
    [
        Flashcard {
            id: "foo".into(),
            front_side: FlashcardSide {
                text: "Foo front".into(),
                language: Language::Ukranian,
            },
            back_side: FlashcardSide {
                text: "Foo back".into(),
                language: Language::Engish,
            },
        }
        .into(),
        Flashcard {
            id: "bar".into(),
            front_side: FlashcardSide {
                text: "Bar front".into(),
                language: Language::Ukranian,
            },
            back_side: FlashcardSide {
                text: "Bar back".into(),
                language: Language::Engish,
            },
        }
        .into(),
    ].into()
}
