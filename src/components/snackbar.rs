use super::common;
use implicit_clone::ImplicitClone;
use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};
use yew::prelude::*;
use yew_autoprops::*;

#[derive(PartialEq, Clone)]
pub struct Message {
    id: usize,
    pub text: AttrValue,
}

impl Message {
    pub fn new(text: impl Into<AttrValue>) -> Self {
        static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: CURRENT_ID.fetch_add(1, Ordering::AcqRel),
            text: text.into(),
        }
    }
}

impl<T> From<T> for Message
where
    T: Into<AttrValue>,
{
    fn from(text: T) -> Self {
        Self::new(text)
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct SnackbarMessage(Option<Message>);

impl Reducible for SnackbarMessage {
    type Action = Message;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::make_mut(&mut self).0 = Some(action);
        self
    }
}

impl ImplicitClone for SnackbarMessage {}

#[autoprops]
#[function_component(Snackbar)]
pub fn snackbar(message: SnackbarMessage) -> Html {
    let message = message.0;
    let (node_ref, comp_ref) = common::use_mdc_init(common::mdc_snackbar);

    use_effect_with(message.clone(), move |message| {
        if let Some(c) = comp_ref.borrow().as_deref() {
            c.close();
            if message.is_some() {
                c.open();
            }
        };
    });

    html! {
      <aside class="mdc-snackbar" ref={node_ref}>
        <div class="mdc-snackbar__surface" role="status">
          <div class="mdc-snackbar__label" aria-atomic="false">
            {message.map(|m| m.text)}
          </div>
        </div>
      </aside>
    }
}
