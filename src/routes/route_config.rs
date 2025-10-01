use leptos::prelude::*;
use leptos_router::components::{Route, Routes};
use leptos_router::path;

use crate::pages::about::About;
use crate::pages::academy::Academy;
use crate::pages::home::Home;

#[component]
pub fn RouteConfig() -> impl IntoView {
    view! {
        <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
            <Route path=path!("/") view=Home/>
            <Route path=path!("/about") view=About/>
            <Route path=path!("/academy") view=Academy/>
            <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
    }
}
