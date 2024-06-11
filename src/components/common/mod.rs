pub mod button;

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

/// Material Design Components
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["mdc", "ripple", "MDCRipple"], js_name = attachTo)]
    fn mdc_ripple(element: HtmlElement);
}

#[hook]
fn use_mdc_init<F>(constructor: F) -> NodeRef
where
    F: Fn(HtmlElement) + 'static,
{
    let node_ref = use_node_ref();
    use_effect_with(node_ref.clone(), move |node_ref| {
        let Some(node) = node_ref.cast() else {
            return;
        };
        constructor(node)
    });
    node_ref
}
