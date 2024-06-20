use super::common::button::{FloatingActionButton, Button};
use crate::model::{
    self,
    flashcard::{Flashcard, FlashcardSide},
    training::LearningSet,
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

#[derive(Default, Clone)]
struct TrainingState {
    learning_set: LearningSet,
}

impl TrainingState {
    fn new(learning_set: LearningSet) -> Self {
        Self { learning_set }
    }
}

enum TraningAction {
    Next,
}

impl Reducible for TrainingState {
    type Action = TraningAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use TraningAction::*;
        let this = Rc::make_mut(&mut self);

        match action {
            Next => this.learning_set.rotate_left(1),
        };
        self
    }
}

#[function_component(Training)]
pub fn training() -> Html {
    let state =
        use_reducer(|| TrainingState::new(model::training::test_flashcards()));

    let move_next = {
        let dispatcher = state.dispatcher();
        move |_| dispatcher.dispatch(TraningAction::Next)
    };
    let Some(card) = state.learning_set.front().cloned() else {
        return "No cards".into();
    };

    html! {
      <div class="training">
        <CurrentCard {card} />
        <div class="controls">
          <Button onclick={move_next}>
            {"Next"}
          </Button>
        </div>
      </div>
    }
}
