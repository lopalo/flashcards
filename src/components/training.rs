use super::{
    common::button::{Button, ButtonVariant, FloatingActionButton},
    context::{LearningSetCtx, SettingsCtx},
};
use crate::{
    model::{
        flashcard::FlashcardSide, learning_set, Flashcard, LearningSetAction,
        Settings,
    },
    speech_synthesis,
};
use std::{
    ops::{Bound, RangeBounds},
    rc::Rc,
};
use web_sys::Element;
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

    fn flip(&mut self) {
        *self = match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
        }
    }
}

#[derive(Clone)]
struct SelectedWords((Bound<usize>, Bound<usize>));

impl SelectedWords {
    fn new(word_index: usize) -> Self {
        let bound = Bound::Included(word_index);
        Self((bound, bound))
    }

    fn all() -> Self {
        Self((Bound::Included(0), Bound::Unbounded))
    }

    fn update(&mut self, word_index: usize) {
        let (
            Bound::Included(start_word_index),
            Bound::Included(end_word_index),
        ) = self.0
        else {
            return;
        };
        let bound = Bound::Included(word_index);
        if word_index < start_word_index {
            self.0 .0 = bound;
        }
        if word_index > end_word_index {
            self.0 .1 = bound
        }
    }
}

#[derive(Default, Clone)]
enum Speech {
    #[default]
    Inactive,
    TextSelection(SelectedWords),
    Active(SelectedWords, speech_synthesis::SpeechGuard),
}

#[derive(Default, Clone)]
struct CurrentCardState {
    card_side: CardSide,
    speech: Speech,
}

enum CurrentCardAction {
    Reset,
    Flip,
    SelectText { word_index: usize },
    UnselectText,
    ActivateSpeech(SelectedWords, speech_synthesis::SpeechGuard),
    DeactivateSpeech,
}

impl Reducible for CurrentCardState {
    type Action = CurrentCardAction;
    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use CurrentCardAction::*;
        let this = Rc::make_mut(&mut self);

        match action {
            UnselectText | ActivateSpeech(..) => {}
            Reset | Flip | SelectText { .. } | DeactivateSpeech => {
                speech_synthesis::deactivate()
            }
        }

        match action {
            Reset => *this = Default::default(),
            Flip => {
                this.card_side.flip();
                this.speech = Speech::Inactive
            }
            SelectText { word_index } => {
                if let Speech::TextSelection(ref mut selected_words) =
                    this.speech
                {
                    selected_words.update(word_index)
                } else {
                    this.speech =
                        Speech::TextSelection(SelectedWords::new(word_index));
                };
            }
            UnselectText => {
                if let Speech::TextSelection(..) = this.speech {
                    this.speech = Speech::Inactive;
                };
            }
            ActivateSpeech(selected_words, speech_guard) => {
                this.speech = Speech::Active(selected_words, speech_guard);
            }
            DeactivateSpeech => {
                if let Speech::Active(_, speech_guard) =
                    std::mem::replace(&mut this.speech, Speech::Inactive)
                {
                    drop(speech_guard)
                }
            }
        }

        self
    }
}

fn activate_speech(
    settings: &Settings,
    state: UseReducerHandle<CurrentCardState>,
    card: &Flashcard,
    selected_words: &SelectedWords,
) {
    let card_side = state.card_side.flashcard_side(card);
    let text = card_side.words()[selected_words.0].join(" ");
    let dispatcher = state.dispatcher();
    let speech_guard = speech_synthesis::activate(
        settings.get_voice(card_side.language),
        &text,
        move || dispatcher.dispatch(CurrentCardAction::DeactivateSpeech),
    );
    state.dispatch(CurrentCardAction::ActivateSpeech(
        selected_words.clone(),
        speech_guard,
    ));
}

#[autoprops]
#[function_component(CurrentCard)]
pub fn current_card(card: Rc<Flashcard>) -> Html {
    let settings: SettingsCtx = use_context().unwrap();
    let state = use_reducer(CurrentCardState::default);

    use_effect_with(card.id.clone(), {
        let dispatcher = state.dispatcher();
        move |_| dispatcher.dispatch(CurrentCardAction::Reset)
    });

    let selected_words = match &state.speech {
        Speech::TextSelection(selected_words)
        | Speech::Active(selected_words, _) => Some(selected_words),
        Speech::Inactive => None,
    };

    let words = state.card_side.flashcard_side(&card).words();
    let words = words.iter().enumerate().map(|(word_index, word)| {
        let mut variant = ButtonVariant::Text;
        if let Some(SelectedWords(selected)) = selected_words {
            if selected.contains(&word_index) {
                variant = ButtonVariant::Outlined;
            }
        }
        let onpointerdown = {
            let dispatcher = state.dispatcher();
            move |e: PointerEvent| {
                e.target_unchecked_into::<Element>()
                    .release_pointer_capture(e.pointer_id())
                    .unwrap();
                dispatcher
                    .dispatch(CurrentCardAction::SelectText { word_index })
            }
        };
        let onpointerenter = {
            let state = state.clone();
            move |_| {
                if let Speech::TextSelection(_) = state.speech {
                } else {
                    return;
                };
                state.dispatch(CurrentCardAction::SelectText { word_index })
            }
        };
        let onpointerup = {
            let settings = settings.clone();
            let state = state.clone();
            let card = card.clone();
            move |_| {
                let Speech::TextSelection(ref selected_words) = state.speech
                else {
                    return;
                };
                activate_speech(&settings, state.clone(), &card, selected_words)
            }
        };
        html! {
          <button
            key={word_index}
            class={classes!("mdc-button", variant.css_class())}
            {onpointerdown}
            {onpointerenter}
            {onpointerup}
          >
            {word}
          </button>
        }
    });

    let flip = {
        let dispatcher = state.dispatcher();
        move |_| dispatcher.dispatch(CurrentCardAction::Flip)
    };
    let speak_all = {
        let settings = settings.clone();
        let state = state.clone();
        let card = card.clone();
        move |_| {
            activate_speech(
                &settings,
                state.clone(),
                &card,
                &SelectedWords::all(),
            )
        }
    };
    let unselect_text = {
        let dispatcher = state.dispatcher();
        move |_| dispatcher.dispatch(CurrentCardAction::UnselectText)
    };

    html! {
      <div class="mdc-card flashcard">
        <div class="card-text" onpointerleave={unselect_text}>
          {for words}
        </div>
        <div class="controls">
          <FloatingActionButton icon_name="autorenew" on_click={flip} />
          <FloatingActionButton icon_name="hearing" on_click={speak_all} />
        </div>
      </div>
    }
}

#[function_component(Training)]
pub fn training() -> Html {
    let settings: SettingsCtx = use_context().unwrap();
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

    let repeat = {
        let dispatcher = learning_set.dispatcher();
        move |_| {
            dispatcher.dispatch(LearningSetAction::MoveHeadCardForward {
                positions: settings.repeat_card_distance,
            })
        }
    };

    let Some(card) = learning_set.queue.front().cloned() else {
        return "No cards".into();
    };

    html! {
      <div class="training">
        <CurrentCard {card} />
        <div class="controls">
          <Button on_click={go_back}>
            {"Back"}
          </Button>
          <Button on_click={repeat}>
            {"Repeat"}
          </Button>
          <Button on_click={go_next}>
            {"Next"}
          </Button>
        </div>
      </div>
    }
}
