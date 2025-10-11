use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Routes};
use leptos_router::path;

use crate::layouts::base::BaseLayout;
use crate::pages::{about::About, academy::Academy, home::Home};

#[component]
pub fn RouteConfig() -> impl IntoView {
    view! {
        <Routes fallback={|| view! { <h1>"Not Found"</h1> }}>
            // root /
            <ParentRoute path={path!("/")} view={BaseLayout}>
                <Route path={path!("/")} view={Home} />
                <Route path={path!("/about")} view={About} />
                <Route path={path!("/academy")} view={Academy} />
            </ParentRoute>

            // Catch-all fallback
            <Route path={path!("/*any")} view={|| view! { <h1>"Not Found"</h1> }} />
        </Routes>
    }
}
