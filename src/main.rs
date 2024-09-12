mod components;
mod local_storage;
mod model;
mod routes;
mod speech_synthesis;

use components::app::App;
use wasm_bindgen::JsValue;
use web_sys::OrientationLockType;

fn main() {
    let _ = lock_landscape_orientation();
    yew::Renderer::<App>::new().render();
}

fn lock_landscape_orientation() -> Result<(), JsValue> {
    let _ = gloo::utils::window()
        .screen()?
        .orientation()
        .lock(OrientationLockType::Landscape);
    Ok(())
}
