use super::{
    common::{page::Page, Trigger},
    learning_set::LearningSet,
    navigation::NavigationDrawer,
    not_found::NotFound,
    settings::Settings,
    training::Training,
};
use crate::routes::Route;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    let show_navigation = use_reducer(Trigger::default);
    let route_switch = {
        let dispatcher = show_navigation.dispatcher();
        move |r| {
            let dispatcher = dispatcher.clone();
            route_switch(move |()| dispatcher.dispatch(()), r)
        }
    };

    let navigation_routes: Rc<[Route]> = use_memo((), |()| {
        [Route::Training, Route::LearningSet, Route::Settings]
    });

    html! {
      <div>
        <BrowserRouter>
          <NavigationDrawer open={*show_navigation} routes={navigation_routes} />
          <Switch<Route> render={route_switch} />
        </BrowserRouter>
      </div>
    }
}

fn route_switch(show_navigation: impl Fn(()) + 'static, route: Route) -> Html {
    let page_title = route.page_title();

    let page_comp = match route {
        Route::Home => html! { <Redirect<Route> to={Route::Training} /> },
        Route::Training => html! { <Training /> },
        Route::LearningSet => html! { <LearningSet /> },
        Route::Settings => html! { <Settings /> },
        Route::NotFound => html! { <NotFound /> },
    };

    html! {
      <div>
        <Page title={page_title} {show_navigation}>
          {page_comp}
        </Page>
      </div>
    }
}
