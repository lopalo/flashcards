use super::{super::context::DisplayErrorCtx, dialog::Dialog, Trigger};
use gloo::file::{callbacks::FileReader, Blob, ObjectUrl};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlInputElement};
use yew::prelude::*;
use yew_autoprops::autoprops;

pub fn import_json<T, F, D>(
    input: HtmlInputElement,
    callback: F,
    display_error: D,
) -> Option<FileReader>
where
    T: DeserializeOwned,
    F: FnOnce(T) + 'static,
    D: Fn(String) + 'static,
{
    let file = input.files()?.get(0)?;
    let reader =
        gloo::file::callbacks::read_as_text(&file.into(), move |payload| {
            let json = match payload {
                Ok(json) => json,
                Err(err) => {
                    return display_error(format!(
                        "Failed to read the file: {err}"
                    ));
                }
            };
            match serde_json::from_str(&json) {
                Ok(value) => callback(value),
                Err(err) => display_error(format!(
                    "Failed to deserialize the file content: {err}"
                )),
            }
        });
    Some(reader)
}

pub fn export_json<T>(file_name: &str, data: T)
where
    T: Serialize,
{
    let blob = Blob::new(serde_json::to_string_pretty(&data).unwrap().as_str());
    let url = ObjectUrl::from(blob);
    let anchor: HtmlAnchorElement = gloo::utils::document()
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();
    anchor.set_download(file_name);
    anchor.set_href(&url);
    let body = gloo::utils::body();
    body.append_child(&anchor).unwrap();
    anchor.click();
    body.remove_child(&anchor).unwrap();
}

#[autoprops]
#[function_component(ImportDialog)]
pub fn import_dialog<T>(
    open: Trigger,
    #[prop_or("Import".into())] title: AttrValue,
    on_import: Callback<T>,
) -> Html
where
    T: DeserializeOwned + 'static,
{
    let display_error: DisplayErrorCtx = use_context().unwrap();
    let file_reader = use_mut_ref(|| None);
    let input_ref = use_node_ref();

    let on_accept = {
        let input_ref = input_ref.clone();
        move |_| {
            let Some(file) = input_ref.cast::<HtmlInputElement>() else {
                return;
            };
            let on_import = on_import.clone();
            let display_error = display_error.clone();
            *file_reader.borrow_mut() = import_json(
                file,
                move |data| on_import.emit(data),
                move |error| display_error.dispatch(error.into()),
            );
        }
    };
    html! {
      <Dialog {open} {title} {on_accept}>
        <input
          type="file"
          accept="application/json"
          ref={input_ref}
        />
      </Dialog>
    }
}
