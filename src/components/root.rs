use super::training::Training;
use yew::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <div>
            <Training />
        </div>
    }
}
