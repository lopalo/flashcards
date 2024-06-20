use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Page)]
pub fn page(
    title: &AttrValue,
    children: &Html,
    show_navigation: Callback<()>,
) -> Html {
    let node_ref = super::use_mdc_init(super::mdc_top_app_bar).0;

    html! {
      <div>
        <header class=" mdc-top-app-bar mdc-top-app-bar--short" ref={node_ref}>
          <div class="mdc-top-app-bar__row">
            <section class="mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
              <button
                class="mdc-icon-button material-icons mdc-top-app-bar__navigation-icon"
                style="--mdc-ripple-color: white"
                onclick={move |_| show_navigation.emit(())}
              >
                <span class="mdc-icon-button__ripple"></span>
                {"menu"}
              </button>
              <span class="mdc-top-app-bar__title">{title}</span>
            </section>
          </div>
        </header>
        <main class="mdc-top-app-bar--short-fixed-adjust">
          {children.clone()}
        </main>
      </div>
    }
}
