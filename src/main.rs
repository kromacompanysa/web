use leptos::prelude::*;
use leptos_router::components::Router;

mod components;
mod layouts;
mod pages;
mod routes;

use crate::routes::route_config::RouteConfig;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <Router>
                <RouteConfig />
            </Router>
        }
    });
}
