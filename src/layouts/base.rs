use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn BaseLayout() -> impl IntoView {
    view! {
        <div class="app-container">
            <header>
                <nav>
                    <a href="/">"Home"</a>
                    <a href="/about">"About"</a>
                    <a href="/academy">"Academy"</a>
                </nav>
            </header>

            <main>
                <Outlet />
            </main>

            <footer>
                <p>"© 2025 My Leptos App"</p>
            </footer>
        </div>
    }
}
