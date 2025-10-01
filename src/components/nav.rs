use leptos::prelude::*;

#[component]
pub fn HeadNav() -> impl IntoView {
    view! {
        <nav>
            <a href="/">"Home"</a>
            <a href="/about">"About"</a>
            <a href="/academy">"Academy"</a>
        </nav>
    }
}
