use super::{
    common::{page::Page, Trigger},
    context::{LearningSetCtx, SettingsCtx},
    learning_set::LearningSet,
    navigation::NavigationDrawer,
    not_found::NotFound,
    settings::Settings,
    training::Training,
};
use crate::{local_storage::LocalStorageRecord, model, routes::Route};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
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

    let settings = use_reducer(model::Settings::restore_from_local_storage);
    let learning_set =
        use_reducer(model::LearningSet::restore_from_local_storage);

    html! {
      <div>
        <BrowserRouter>
        <ContextProvider<SettingsCtx> context={settings}>
        <ContextProvider<LearningSetCtx> context={learning_set}>
          <NavigationDrawer open={*show_navigation} routes={navigation_routes} />
          <Switch<Route> render={route_switch} />
        </ContextProvider<LearningSetCtx>>
        </ContextProvider<SettingsCtx>>
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
