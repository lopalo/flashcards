use yew::prelude::*;
#[allow(unused)]
#[derive(PartialEq)]
pub enum ButtonVariant {
    Text,
    Raised,
    Outlined,
}

impl ButtonVariant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Text => "",
            Self::Raised => "mdc-button--raised",
            Self::Outlined => "mdc-button--outlined",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub onclick: Callback<MouseEvent>,
    pub children: Html,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        variant,
        onclick,
        children,
    } = props;

    let node_ref = super::use_mdc_init(super::mdc_ripple);

    html! {
        <button
            class={classes!("mdc-button", variant.css_class())}
            ref={node_ref}
            {onclick}
        >
            <span class="mdc-button__ripple"></span>
            { children.clone() }
        </button>
    }
}
