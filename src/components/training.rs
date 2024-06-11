use super::common::button::{Button, ButtonVariant};
use crate::model::{
    self,
    flashcard::{Flashcard, FlashcardSide},
};
use std::rc::Rc;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[derive(Default, Clone, Copy)]
enum CardSide {
    #[default]
    Front,
    Back,
}

impl CardSide {
    fn flashcard_side(self, flashcard: &Flashcard) -> &FlashcardSide {
        match self {
            Self::Front => &flashcard.front_side,
            Self::Back => &flashcard.back_side,
        }
    }

    fn flip(&self) -> Self {
        match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
        }
    }
}

#[derive(Default, Clone)]
struct CurrentCardState {
    card_side: CardSide,
}

#[autoprops]
#[function_component(CurrentCard)]
pub fn current_card(card: Rc<Flashcard>) -> Html {
    let state = use_state(CurrentCardState::default);

    let card_text = state.card_side.flashcard_side(&card).text.clone();

    let flip = {
        let state = state.clone();
        move |_| {
            let card_side = state.card_side.flip();
            state.set(CurrentCardState {
                card_side,
            });
        }
    };

    use_effect_with(card.id.clone(), move |_| state.set(Default::default()));

    html! {
        <div class="mdc-card flashcard">
            <div class="card-text">
                { card_text }
            </div>
            <div class="flip-btn">
                <Button variant={ButtonVariant::Raised} onclick={flip}>
                    { "Flip" }
                </Button>
            </div>
        </div>
    }
}

#[derive(Default)]
struct TrainingState {
    current_card_idx: usize,
}

#[function_component(Training)]
pub fn training() -> Html {
    let flashcards = use_state(model::training::test_flashcards);
    let state = use_state(TrainingState::default);

    let move_next = {
        let flashcards = flashcards.clone();
        let state = state.clone();
        move |_| {
            let mut current_card_idx = state.current_card_idx + 1;
            if current_card_idx >= flashcards.len() {
                current_card_idx = 0;
            }
            state.set(TrainingState {
                current_card_idx,
            });
        }
    };
    if flashcards.is_empty() {
        return "No cards".into();
    }

    html! {
        <div class="training">
            <CurrentCard card={flashcards[state.current_card_idx].clone()} />
            <div class="controls">
                <Button variant={ButtonVariant::Raised} onclick={move_next}>
                    { "Next" }
                </Button>
            </div>
        </div>
    }
}
