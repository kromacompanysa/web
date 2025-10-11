use leptos::prelude::*;

use crate::components::carousel::BaseCarousel;
use crate::components::grid::BaseGrid;
use crate::components::hero::BaseHero;
use crate::components::pre_footer::BasePreFooter;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <BaseHero />
        <BaseCarousel />
        <BaseGrid />
        <BasePreFooter />
    }
}
