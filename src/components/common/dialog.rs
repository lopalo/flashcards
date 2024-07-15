use super::Trigger;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Dialog)]
pub fn dialog(
    open: Trigger,
    title: AttrValue,
    children: &Html,
    on_accept: Callback<()>,
) -> Html {
    let (node_ref, comp_ref) = super::use_mdc_init(super::mdc_dialog);
    super::use_trigger(open, move || {
        if let Some(c) = comp_ref.borrow().as_deref() {
            c.open()
        };
    });

    html! {
      <div class="mdc-dialog mdc-dialog--fullscreen" ref={node_ref}>
        <div class="mdc-dialog__container">
          <div class="mdc-dialog__surface" role="alertdialog">
            <div class="mdc-dialog__header">
              <h2 class="mdc-dialog__title">
                {title}
              </h2>
              <button
                class="mdc-icon-button material-icons mdc-dialog__close"
                data-mdc-dialog-action="close"
              >
                {"close"}
              </button>
            </div>
            <div class="mdc-dialog__content">
              {children.clone()}
            </div>
            <div class="mdc-dialog__actions">
              <button
                type="button"
                class="mdc-button mdc-dialog__button"
                data-mdc-dialog-action="close"
              >
                <div class="mdc-button__ripple"></div>
                <span class="mdc-button__label">{"Cancel"}</span>
              </button>
              <button
                type="button"
                class="mdc-button mdc-dialog__button"
                data-mdc-dialog-action="accept"
                onclick={move |_| on_accept.emit(())}
              >
                <div class="mdc-button__ripple"></div>
                <span class="mdc-button__label">{"OK"}</span>
              </button>
            </div>
          </div>
        </div>
        <div class="mdc-dialog__scrim"></div>
      </div>
    }
}
