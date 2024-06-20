mod components;
mod model;
mod routes;

use components::root::Root;

fn main() {
    yew::Renderer::<Root>::new().render();
}
