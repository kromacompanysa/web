use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::footer::BaseFooter;
use crate::components::header::BaseHeader;

#[component]
pub fn BaseLayout() -> impl IntoView {
    view! {
        <div class="app-container">

            <BaseHeader />

            <main>
                <Outlet />
            </main>

            <BaseFooter />
        </div>
    }
}
