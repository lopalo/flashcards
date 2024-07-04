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
    pub fn css_class(&self) -> &'static str {
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
    pub on_click: Callback<MouseEvent>,
    pub children: Html,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        variant,
        on_click,
        children,
    } = props;

    let node_ref = super::use_mdc_init(super::mdc_ripple).0;

    html! {
      <button
        class={classes!("mdc-button", variant.css_class())}
        ref={node_ref}
        onclick={on_click}
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
    #[prop_or_default]
    pub mini: bool,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(FloatingActionButton)]
pub fn floating_action_button(props: &FloatingActionButtonProps) -> Html {
    let FloatingActionButtonProps {
        icon_name,
        mini,
        on_click,
    } = props;
    let node_ref = super::use_mdc_init(super::mdc_ripple).0;
    html! {
      <button
        class={classes!("mdc-fab", mini.then_some("mdc-fab--mini"))}
        ref={node_ref}
        onclick={on_click}
      >
        <div class="mdc-fab__ripple"></div>
        <span class="mdc-fab__icon material-icons">{icon_name}</span>
      </button>
    }
}
