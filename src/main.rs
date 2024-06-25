mod components;
mod model;
mod routes;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
