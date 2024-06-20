pub mod button;
pub mod page;

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
            *component_ref.borrow_mut() = component.as_ref().map(Rc::clone);
            move || {
                component.map(|c| c.as_ref().as_ref().destroy());
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

impl html::ImplicitClone for Trigger {}

impl Reducible for Trigger {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        self.incr().into()
    }
}
