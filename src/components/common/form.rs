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
    #[prop_or_default] helper_text: Option<AttrValue>,
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

#[autoprops]
#[function_component(Textarea)]
pub fn textarea(
    label: AttrValue,
    #[prop_or_default] helper_text: Option<AttrValue>,
    #[prop_or(3)] rows: usize,
    #[prop_or(40)] cols: usize,
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
          class="mdc-text-field mdc-text-field--outlined mdc-text-field--textarea"
          ref={node_ref}
        >
          <span class="mdc-notched-outline">
            <span class="mdc-notched-outline__leading"></span>
              <span class="mdc-notched-outline__notch">
                <span class="mdc-floating-label">{label}</span>
              </span>
            <span class="mdc-notched-outline__trailing"></span>
          </span>
          <span class="mdc-text-field__resizer">
            <textarea
              class="mdc-text-field__input"
              rows={rows.to_string()}
              cols={cols.to_string()}
              value={value}
              {oninput}
            />
          </span>
        </label>
        if let Some(helper_text) = helper_text {
          <div class="mdc-text-field-helper-line">
            <div class="mdc-text-field-helper-text">{helper_text}</div>
          </div>
        }
      </div>
    }
}

#[autoprops]
#[function_component(Checkbox)]
pub fn checkbox(
    #[prop_or_default] id: Option<AttrValue>,
    label: AttrValue,
    checked: bool,
    on_change: Callback<bool>,
) -> Html {
    let node_ref = super::use_mdc_init(super::mdc_checkbox).0;
    let oninput = move |e: InputEvent| {
        on_change.emit(e.target_unchecked_into::<HtmlInputElement>().checked())
    };

    html! {
      <div class="mdc-form-field">
        <div class="mdc-checkbox" ref={node_ref}>
          <input
            id={id.clone()}
            type="checkbox"
            class="mdc-checkbox__native-control"
            {checked}
            {oninput}
          />
          <div class="mdc-checkbox__background">
            <svg class="mdc-checkbox__checkmark" viewBox="0 0 24 24">
              <path
                class="mdc-checkbox__checkmark-path"
                fill="none"
                d="M1.73,12.91 8.1,19.28 22.79,4.59" />
            </svg>
            <div class="mdc-checkbox__mixedmark"></div>
          </div>
          <div class="mdc-checkbox__ripple" />
        </div>
        <label for={id}>{label}</label>
      </div>
    }
}
