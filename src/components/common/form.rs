use implicit_clone::ImplicitClone;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[derive(PartialEq, Default, Clone, Copy)]
pub enum TextFieldVariant {
    #[default]
    Text,
    Number,
}

impl TextFieldVariant {
    pub fn input_type(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
        }
    }
}

impl ImplicitClone for TextFieldVariant {}

#[autoprops]
#[function_component(TextField)]
pub fn text_field(
    #[prop_or_default] variant: TextFieldVariant,
    label: AttrValue,
    helper_text: Option<AttrValue>,
    value: AttrValue,
    on_change: Callback<String>,
) -> Html {
    let node_ref = super::use_mdc_init(super::mdc_text_field).0;
    let oninput = move |e: InputEvent| {
        on_change.emit(e.target_unchecked_into::<HtmlInputElement>().value())
    };

    html! {
      <div>
        <label
          class="mdc-text-field mdc-text-field--outlined"
          ref={node_ref}
        >
          <span class="mdc-notched-outline">
            <span class="mdc-notched-outline__leading"></span>
            <span class="mdc-notched-outline__notch">
              <span class="mdc-floating-label">{label}</span>
            </span>
            <span class="mdc-notched-outline__trailing"></span>
          </span>
          <input
            type={variant.input_type()}
            class="mdc-text-field__input"
            value={value}
            {oninput}
          />
        </label>
        if let Some(helper_text) = helper_text {
          <div class="mdc-text-field-helper-line">
            <div class="mdc-text-field-helper-text">{helper_text}</div>
          </div>
        }
      </div>
    }
}
