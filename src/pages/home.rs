use leptos::prelude::*;

use crate::components::carousel::BaseCarousel;
use crate::components::grid::BaseGrid;
use crate::components::hero::BaseHero;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <BaseHero/>
        <BaseCarousel/>
        <BaseGrid />
    }
}
