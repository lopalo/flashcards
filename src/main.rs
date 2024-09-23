mod components;
mod local_storage;
mod model;
mod routes;
mod speech_synthesis;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
