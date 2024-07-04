use super::{dialog::Dialog, Trigger};
use gloo::file::{callbacks::FileReader, Blob, ObjectUrl};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlInputElement};
use yew::prelude::*;
use yew_autoprops::autoprops;

pub fn import_json<T, F>(
    input: HtmlInputElement,
    callback: F,
) -> Option<FileReader>
where
    T: DeserializeOwned,
    F: FnOnce(T) + 'static,
{
    let file = input.files()?.get(0)?;
    let reader = gloo::file::callbacks::read_as_text(&file.into(), |payload| {
        //TODO: show a snackbar with the errors
        let json = payload.expect("Failed to read the file");
        let value = serde_json::from_str(&json)
            .expect("Failed to deserialize the file content");
        callback(value)
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
    let file_reader = use_mut_ref(|| None);

    let input_ref = use_node_ref();
    let on_accept = {
        let input_ref = input_ref.clone();
        move |_| {
            let Some(file) = input_ref.cast::<HtmlInputElement>() else {
                return;
            };
            let on_import = on_import.clone();
            *file_reader.borrow_mut() =
                import_json(file, move |data| on_import.emit(data));
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
