use super::common::{self, Trigger};
use crate::routes::Route;
use std::rc::Rc;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::prelude::*;

#[autoprops]
#[function_component(NavigationDrawer)]
pub fn navigation_drawer(open: Trigger, routes: Rc<[Route]>) -> Html {
    let (node_ref, comp_ref) = common::use_mdc_init(common::mdc_drawer);
    common::use_trigger(open, move || {
        if let Some(c) = comp_ref.borrow().as_deref() {
            c.set_open(true)
        }
    });

    let current_route = use_route();

    let navigation_items = routes.iter().map(|route| {
        let is_active = current_route.as_ref() == Some(route);
        let classes = classes!(
            "mdc-deprecated-list-item",
            is_active.then_some("mdc-deprecated-list-item--activated")
        );
        let icon = route.navigation_icon();
        html! {
          <Link<Route> to={route} {classes}>
            <span class="mdc-deprecated-list-item__ripple"></span>
            <i class="material-icons mdc-deprecated-list-item__graphic" aria-hidden="true">
              {icon}
            </i>
            <span class="mdc-deprecated-list-item__text">
              {route.page_title()}
            </span>
          </Link<Route>>
        }
    });

    html! {
      <>
        <aside class="mdc-drawer mdc-drawer--modal" ref={node_ref}>
          <div class="mdc-drawer__content">
            <nav class="mdc-deprecated-list">
              {for navigation_items}
            </nav>
          </div>
        </aside>

        <div class="mdc-drawer-scrim"></div>
      </>
    }
}
