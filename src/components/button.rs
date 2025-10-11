use leptos::prelude::*;

#[component]
pub fn PrimaryButton(label: String) -> impl IntoView {
    view! { <button class="primary-btn">{label}</button> }
}
