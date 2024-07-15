use super::{
    common::{
        self,
        button::{Button, FloatingActionButton},
        dialog::Dialog,
        dropdown::Dropdown,
        file::{self, ImportDialog},
        form::Textarea,
        Trigger,
    },
    context::{LearningSetCtx, SettingsCtx},
};
use crate::model::{
    self,
    flashcard::{Flashcard, FlashcardAction, Language},
    LearningSetAction,
};
use std::rc::Rc;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[function_component(LearningSet)]
pub fn learning_set() -> Html {
    let learning_set: LearningSetCtx = use_context().unwrap();
    let show_add_card_form = use_reducer(Trigger::default);
    let show_edit_card_form = use_reducer(Trigger::default);
    let card_to_edit = use_state(|| None);
    let show_import_dialog = use_reducer(Trigger::default);

    let on_add_card_click = {
        let dispatcher = show_add_card_form.dispatcher();
        move |_| dispatcher.dispatch(())
    };

    let on_edit_card_click = {
        let card_to_edit = card_to_edit.clone();
        let dispatcher = show_edit_card_form.dispatcher();
        move |card| {
            card_to_edit.set(Some(card));
            dispatcher.dispatch(())
        }
    };

    let on_import_click = {
        let dispatcher = show_import_dialog.dispatcher();
        move |_| dispatcher.dispatch(())
    };

    let on_import = {
        let dispatcher = learning_set.dispatcher();
        move |data| dispatcher.dispatch(LearningSetAction::Replace(data))
    };

    let on_export_click = {
        let learning_set = learning_set.clone();
        move |_| file::export_json("learning_set.json", &*learning_set)
    };

    let items = learning_set
        .queue
        .iter()
        .cloned()
        .map(|card| html! {
            <FlashcardRecord {card} on_edit_card_click={on_edit_card_click.clone()} />
        });

    html! {
      <div class="learning-set">
        <AddFlashcardForm open={*show_add_card_form} />
        if let Some(card_to_edit) = (*card_to_edit).clone() {
            <EditFlashcardForm open={*show_edit_card_form} card={card_to_edit}/>
        }
        <ImportDialog<model::LearningSet> open={*show_import_dialog} {on_import}/>
        <div class="controls">
          <Button on_click={on_import_click}>
            {"Import"}
          </Button>
          <Button on_click={on_export_click}>
            {"Export"}
          </Button>
        </div>
        <ul class="mdc-deprecated-list mdc-deprecated-list--two-line mdc-deprecated-list--dense">
          {for items}
        </ul>
        <div class="add-flashcard-button">
          <FloatingActionButton icon_name="add" on_click={on_add_card_click} />
        </div>
      </div>
    }
}

#[autoprops]
#[function_component(FlashcardRecord)]
fn flashcard_record(
    card: Rc<Flashcard>,
    on_edit_card_click: Callback<Rc<Flashcard>>,
) -> Html {
    let learning_set: LearningSetCtx = use_context().unwrap();
    let on_delete_click = {
        let learning_set = learning_set.clone();
        let card_id = card.id.clone();
        move |_| {
            learning_set.dispatch(LearningSetAction::DeleteCard {
                flashcard_id: card_id.clone(),
            })
        }
    };

    let on_edit_click = {
        let card = card.clone();
        move |_| on_edit_card_click.emit(card.clone())
    };

    html! {
      <>
        <li role="separator" class="mdc-deprecated-list-divider" />
        <li key={card.id.as_str()} class="mdc-deprecated-list-item">
          <span class="mdc-deprecated-list-item__graphic material-icons drag-handle">
            {"drag_handle"}
          </span>
          <span class="mdc-deprecated-list-item__text">
            <span class="mdc-deprecated-list-item__primary-text">{card.front_side.text.as_str()}</span>
            <span class="mdc-deprecated-list-item__secondary-text">{card.back_side.text.as_str()}</span>
          </span>
          <span class="mdc-deprecated-list-item__meta">
            <button
              class="mdc-icon-button material-icons mdc-theme--error"
              onclick={on_delete_click}
            >
              {"delete"}
            </button>
            <button
              class="mdc-icon-button material-icons mdc-theme--secondary"
              onclick={on_edit_click}
            >
              {"edit"}
            </button>
          </span>
        </li>
      </>
    }
}

#[autoprops]
#[function_component(AddFlashcardForm)]
fn add_flashcard_form(open: Trigger) -> Html {
    let settings: SettingsCtx = use_context().unwrap();
    let learning_set: LearningSetCtx = use_context().unwrap();
    let card = use_reducer(Flashcard::new);

    common::use_trigger(open, {
        let dispatcher = card.dispatcher();
        move || {
            dispatcher.dispatch(FlashcardAction::ReplaceWithNew);
            dispatcher.dispatch(FlashcardAction::SetFrontLanguage(
                settings.default_card_front_side_language,
            ));
            dispatcher.dispatch(FlashcardAction::SetBackLanguage(
                settings.default_card_back_side_language,
            ));
        }
    });

    let on_select_front_language = {
        let card = card.clone();
        move |lang| card.dispatch(FlashcardAction::SetFrontLanguage(lang))
    };

    let on_change_front_text = {
        let card = card.clone();
        move |text| card.dispatch(FlashcardAction::SetFrontText(text))
    };

    let on_select_back_language = {
        let card = card.clone();
        move |lang| card.dispatch(FlashcardAction::SetBackLanguage(lang))
    };

    let on_change_back_text = {
        let card = card.clone();
        move |text| card.dispatch(FlashcardAction::SetBackText(text))
    };

    let on_accept = {
        let learning_set = learning_set.clone();
        let card = card.clone();
        move |()| {
            learning_set
                .dispatch(LearningSetAction::AppendCard((*card).clone()))
        }
    };

    let languages = Language::all_languages();

    html! {
      <Dialog {open} title="Add flashcard" {on_accept}>
        <div class="mdc-layout-grid">
          <div class="mdc-layout-grid__inner">
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Dropdown<Language>
                label="Front side language"
                items={languages}
                selected={card.front_side.language}
                on_select={on_select_front_language}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Textarea
                label="Front side text"
                value={card.front_side.text.clone()}
                on_change={on_change_front_text}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Dropdown<Language>
                label="Back side language"
                items={languages}
                selected={card.back_side.language}
                on_select={on_select_back_language}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Textarea
                label="Back side text"
                value={card.back_side.text.clone()}
                on_change={on_change_back_text}
              />
            </div>
          </div>
        </div>
      </Dialog>
    }
}

#[autoprops]
#[function_component(EditFlashcardForm)]
fn edit_flashcard_form(open: Trigger, card: Rc<Flashcard>) -> Html {
    let learning_set: LearningSetCtx = use_context().unwrap();
    let edited_card = use_reducer(|| (*card).clone());

    common::use_trigger(open, {
        let dispatcher = edited_card.dispatcher();
        move || {
            dispatcher.dispatch(FlashcardAction::ReplaceWith(card));
        }
    });

    let on_select_front_language = {
        let flashcard = edited_card.clone();
        move |lang| flashcard.dispatch(FlashcardAction::SetFrontLanguage(lang))
    };

    let on_change_front_text = {
        let flashcard = edited_card.clone();
        move |text| flashcard.dispatch(FlashcardAction::SetFrontText(text))
    };

    let on_select_back_language = {
        let flashcard = edited_card.clone();
        move |lang| flashcard.dispatch(FlashcardAction::SetBackLanguage(lang))
    };

    let on_change_back_text = {
        let flashcard = edited_card.clone();
        move |text| flashcard.dispatch(FlashcardAction::SetBackText(text))
    };

    let on_accept = {
        let learning_set = learning_set.clone();
        let flashcard = edited_card.clone();
        move |()| {
            learning_set
                .dispatch(LearningSetAction::ReplaceCard((*flashcard).clone()))
        }
    };

    let languages = Language::all_languages();
    let card_id = &edited_card.id;

    html! {
      <Dialog {open} title={format!("Edit flashcard ({card_id})")} {on_accept}>
        <div class="mdc-layout-grid">
          <div class="mdc-layout-grid__inner">
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Dropdown<Language>
                label="Front side language"
                items={languages}
                selected={edited_card.front_side.language}
                on_select={on_select_front_language}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Textarea
                label="Front side text"
                value={edited_card.front_side.text.clone()}
                on_change={on_change_front_text}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Dropdown<Language>
                label="Back side language"
                items={languages}
                selected={edited_card.back_side.language}
                on_select={on_select_back_language}
              />
            </div>
            <div class="mdc-layout-grid__cell mdc-layout-grid__cell--span-12">
              <Textarea
                label="Back side text"
                value={edited_card.back_side.text.clone()}
                on_change={on_change_back_text}
              />
            </div>
          </div>
        </div>
      </Dialog>
    }
}
