mod components;
mod model;
mod routes;
mod local_storage;
mod speech_synthesis;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
