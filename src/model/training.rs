use super::flashcard::{Flashcard, FlashcardSide, Language};
use std::rc::Rc;

pub fn test_flashcards() -> Vec<Rc<Flashcard>> {
    vec![
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
    ]
}
