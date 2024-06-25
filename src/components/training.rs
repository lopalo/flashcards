use super::{
    common::button::{Button, FloatingActionButton},
    context::LearningSetCtx,
};
use crate::model::{
    flashcard::FlashcardSide, learning_set, Flashcard, LearningSetAction,
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

    let card_text = state.card_side.flashcard_side(&card).text.as_str();

    let flip = {
        let state = state.clone();
        move |_| {
            state.set(CurrentCardState {
                card_side: state.card_side.flip(),
            });
        }
    };

    use_effect_with(card.id.clone(), move |_| state.set(Default::default()));

    html! {
      <div class="mdc-card flashcard">
        <div class="card-text">
          {card_text}
        </div>
        <div class="flip-btn">
          <FloatingActionButton icon_name="autorenew" onclick={flip} />
        </div>
      </div>
    }
}

#[function_component(Training)]
pub fn training() -> Html {
    let learning_set: LearningSetCtx = use_context().unwrap();

    let go_back = {
        let dispatcher = learning_set.dispatcher();
        move |_| {
            dispatcher.dispatch(LearningSetAction::RotateQueue(
                learning_set::Direction::Right,
            ))
        }
    };

    let go_next = {
        let dispatcher = learning_set.dispatcher();
        move |_| {
            dispatcher.dispatch(LearningSetAction::RotateQueue(
                learning_set::Direction::Left,
            ))
        }
    };
    let Some(card) = learning_set.queue.front().cloned() else {
        return "No cards".into();
    };

    html! {
      <div class="training">
        <CurrentCard {card} />
        <div class="controls">
          <Button onclick={go_back}>
            {"Back"}
          </Button>
          //TODO: move a card N items forward
          <Button onclick={|_| {}}>
            {"Repeat"}
          </Button>
          <Button onclick={go_next}>
            {"Next"}
          </Button>
        </div>
      </div>
    }
}
