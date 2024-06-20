use yew::prelude::*;
#[allow(unused)]
#[derive(PartialEq, Default)]
pub enum ButtonVariant {
    Text,
    #[default]
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
    #[prop_or_default]
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

    let node_ref = super::use_mdc_init(super::mdc_ripple).0;

    html! {
      <button
        class={classes!("mdc-button", variant.css_class())}
        ref={node_ref}
        {onclick}
      >
        <span class="mdc-button__ripple"></span>
        {children.clone()}
      </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct FloatingActionButtonProps {
    #[prop_or_default]
    pub icon_name: AttrValue,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(FloatingActionButton)]
pub fn floating_action_button(props: &FloatingActionButtonProps) -> Html {
    let FloatingActionButtonProps { icon_name, onclick } = props;
    let node_ref = super::use_mdc_init(super::mdc_ripple).0;
    html! {
      <button class="mdc-fab" ref={node_ref} {onclick} aria-label="Fooo">
        <div class="mdc-fab__ripple"></div>
        <span class="mdc-fab__icon material-icons">{icon_name}</span>
      </button>
    }
}
