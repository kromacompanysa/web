use leptos::mount::mount_to;
use leptos::prelude::window;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_router::components::Router;

mod components;
mod layouts;
mod pages;
mod routes;

use crate::routes::route_config::RouteConfig;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let document = window().document().expect("no document found");
    let kroma = document
        .get_element_by_id("kroma")
        .expect("no #kroma element found in index.html")
        .unchecked_into::<leptos::web_sys::HtmlElement>();

    mount_to(kroma, || {
        view! {
            <Router>
                <RouteConfig />
            </Router>
        }
    })
    .forget();
}
