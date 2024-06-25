use super::{
    common::{
        button::Button,
        file::{self, ImportDialog},
        Trigger,
    },
    context::LearningSetCtx,
};
use crate::model::{self, LearningSetAction};
use yew::prelude::*;

#[function_component(LearningSet)]
pub fn learning_set() -> Html {
    let learning_set: LearningSetCtx = use_context().unwrap();
    let show_import_dialog = use_reducer(Trigger::default);

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

    let items = learning_set.queue.iter().map(|card| {
        html! {
          <>
            <li role="separator" class="mdc-deprecated-list-divider"></li>
            <li key={card.id.as_str()} class="mdc-deprecated-list-item">
              <span class="mdc-deprecated-list-item__text">
                <span class="mdc-deprecated-list-item__primary-text">{card.front_side.text.as_str()}</span>
                <span class="mdc-deprecated-list-item__secondary-text">{card.back_side.text.as_str()}</span>
              </span>
            </li>
          </>
        }
    });

    html! {
      <div class="learning-set">
        <ImportDialog<model::LearningSet> open={*show_import_dialog} {on_import}/>
        <div class="controls">
          <Button onclick={on_import_click}>
            {"Import"}
          </Button>
          <Button onclick={on_export_click}>
            {"Export"}
          </Button>
        </div>
        <ul class="mdc-deprecated-list mdc-deprecated-list--two-line mdc-deprecated-list--dense">
          {for items}
        </ul>
      </div>
    }
}
