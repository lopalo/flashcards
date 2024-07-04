use implicit_clone::{unsync::IArray, ImplicitClone};
use std::fmt::Display;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(Dropdown)]
pub fn dropdown<T>(
    label: AttrValue,
    items: IArray<T>,
    selected: &T,
    on_select: Callback<T>,
) -> Html
where
    T: 'static + PartialEq + ImplicitClone + Display,
{
    let (node_ref, comp_ref) = super::use_mdc_init(super::mdc_select);
    let items = if items.is_empty() {
        std::slice::from_ref(selected)
    } else {
        items.as_slice()
    };

    use_effect_with(items.to_owned(), {
        let comp_ref = comp_ref.clone();
        move |_| {
            if let Some(c) = comp_ref.borrow().as_deref() {
                c.initialize();
            };
        }
    });

    use_effect_with(selected.clone(), move |selected| {
        if let Some(c) = comp_ref.borrow().as_deref() {
            c.set_value(&selected.to_string());
        };
    });

    let items = items.iter().map(|item| {
        let text = &item.to_string();
        let onclick = {
            let on_select = on_select.clone();
            let item = item.clone();
            move |_| on_select.emit(item.clone())
        };

        html! {
          <li
            class="mdc-deprecated-list-item"
            role="option"
            data-value={text.clone()}
            {onclick}
          >
            <span class="mdc-deprecated-list-item__ripple"></span>
            <span class="mdc-deprecated-list-item__text">
              {text}
            </span>
          </li>
        }
    });

    html! {
      <div class="mdc-select mdc-select--outlined" ref={node_ref}>
        <div class="mdc-select__anchor">
          <span class="mdc-notched-outline">
            <span class="mdc-notched-outline__leading"></span>
            <span class="mdc-notched-outline__notch">
              <span class="mdc-floating-label">{label}</span>
            </span>
            <span class="mdc-notched-outline__trailing"></span>
          </span>
          <span class="mdc-select__selected-text-container">
            <span class="mdc-select__selected-text"></span>
          </span>
          <span class="mdc-select__dropdown-icon">
            <svg
              class="mdc-select__dropdown-icon-graphic"
              viewBox="7 10 10 5" focusable="false"
            >
              <polygon
                class="mdc-select__dropdown-icon-inactive"
                stroke="none"
                fill-rule="evenodd"
                points="7 10 12 15 17 10"
              />
              <polygon
                class="mdc-select__dropdown-icon-active"
                stroke="none"
                fill-rule="evenodd"
                points="7 15 12 10 17 15"
              />
            </svg>
          </span>
        </div>
        <div class="mdc-select__menu mdc-menu mdc-menu-surface mdc-menu-surface--fullwidth">
          <ul class="mdc-deprecated-list" role="listbox">
            {for items}
          </ul>
        </div>
      </div>
    }
}
