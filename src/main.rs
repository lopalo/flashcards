mod components;
mod model;

use components::root::Root;

fn main() {
    yew::Renderer::<Root>::new().render();
}
