#![allow(clippy::empty_docs)]

pub mod button;
pub mod dialog;
pub mod dropdown;
pub mod file;
pub mod form;
pub mod page;

use implicit_clone::ImplicitClone;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

/// Material Design Components
#[wasm_bindgen]
extern "C" {
    pub type MdcComponent;

    // `structural` is needed to call the overridden method of a subclass
    #[wasm_bindgen(method, structural)]
    fn destroy(this: &MdcComponent);

    #[wasm_bindgen(js_namespace = ["mdc", "ripple", "MDCRipple"], js_name = attachTo)]
    fn mdc_ripple(element: HtmlElement) -> MdcComponent;

    #[wasm_bindgen(js_namespace = ["mdc", "topAppBar", "MDCTopAppBar"], js_name = attachTo)]
    fn mdc_top_app_bar(element: HtmlElement) -> MdcComponent;

    #[wasm_bindgen(extends = MdcComponent)]
    pub type MdcDrawer;

    #[wasm_bindgen(js_namespace = ["mdc", "drawer", "MDCDrawer"], js_name = attachTo)]
    pub fn mdc_drawer(element: HtmlElement) -> MdcDrawer;

    #[wasm_bindgen(method, setter = open)]
    pub fn set_open(this: &MdcDrawer, val: bool);

    #[wasm_bindgen(extends = MdcComponent)]
    pub type MdcDialog;

    #[wasm_bindgen(js_namespace = ["mdc", "dialog", "MDCDialog"], js_name = attachTo)]
    pub fn mdc_dialog(element: HtmlElement) -> MdcDialog;

    #[wasm_bindgen(method)]
    pub fn open(this: &MdcDialog);

    #[wasm_bindgen(js_namespace = ["mdc", "textField", "MDCTextField"], js_name = attachTo)]
    fn mdc_text_field(element: HtmlElement) -> MdcComponent;

    #[wasm_bindgen(extends = MdcComponent)]
    pub type MdcSelect;

    #[wasm_bindgen(js_namespace = ["mdc", "select", "MDCSelect"], js_name = attachTo)]
    fn mdc_select(element: HtmlElement) -> MdcSelect;

    #[wasm_bindgen(method)]
    pub fn initialize(this: &MdcSelect);

    #[wasm_bindgen(method, setter = value)]
    pub fn set_value(this: &MdcSelect, value: &str);

}

#[hook]
pub fn use_mdc_init<T, F>(
    constructor: F,
) -> (NodeRef, Rc<RefCell<Option<Rc<T>>>>)
where
    T: AsRef<MdcComponent> + 'static,
    F: Fn(HtmlElement) -> T + 'static,
{
    let node_ref = use_node_ref();
    let component_ref = use_mut_ref(|| None);
    use_effect_with(node_ref.clone(), {
        let component_ref = component_ref.clone();
        move |node_ref| {
            let component = node_ref.cast().map(constructor).map(Rc::new);
            component_ref.borrow_mut().clone_from(&component);
            move || {
                if let Some(c) = component {
                    c.as_ref().as_ref().destroy()
                }
            }
        }
    });
    (node_ref, component_ref)
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Trigger(usize);

impl Trigger {
    pub fn incr(mut self) -> Self {
        self.0 += 1;
        self
    }
}

impl ImplicitClone for Trigger {}

impl Reducible for Trigger {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        self.incr().into()
    }
}

#[hook]
pub fn use_trigger<F>(trigger: Trigger, f: F)
where
    F: FnOnce() + 'static,
{
    use_effect_with(trigger, move |trigger| {
        if *trigger == Trigger::default() {
            return;
        }
        f()
    });
}
