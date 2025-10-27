use leptos::prelude::*;

use crate::components::nav_dropdown::HeaderDropdownMenu;

#[component]
pub fn BaseNav() -> impl IntoView {
    view! {
        <nav class="hidden items-center space-x-8 font-medium text-black md:flex">
            <a href="/about" class="transition-colors hover:text-red-600">
                {"¿Quiénes somos?"}
            </a>

            <HeaderDropdownMenu
                title="Clínica dental"
                items={&[
                    ("/", "Periodoncia"),
                    ("/", "Odontología restauradora"),
                    ("/", "Endodoncia"),
                    ("/", "Ortodoncia"),
                    ("/", "Estética"),
                    ("/", "Rehabilitación oral"),
                    ("/", "Implantología"),
                    ("/", "Cirugía"),
                    ("/", "Prótesis"),
                    ("/", "Odontopediatría"),
                ]}
            />

            <HeaderDropdownMenu
                title="Academia"
                items={&[
                    ("/", "Fotografía odontológica"),
                    ("/", "Prótesis completa"),
                    ("/", "Resinas con guía transparente"),
                    ("/", "Operatoria"),
                    ("/", "Rehabilitación oral"),
                    ("/", "Resinas estratificadas"),
                    ("/", "Endodoncia"),
                    ("/", "Periodoncia e implantes"),
                    ("/", "Ortodoncia"),
                    ("/", "Odontología digital"),
                ]}
            />

            <a href="/contact" class="transition-colors hover:text-red-600">
                {"Contacto"}
            </a>

            <a href="/cart" class="flex relative items-center transition-colors hover:text-red-600">
                <img src="assets/icons/cart4.svg" class="w-5 h-5" alt="Cart" />
                // Optional mini badge (hard-coded 0; wire to a Signal later)
                <span class="absolute -top-2 -right-4 py-0.5 px-1.5 text-xs text-white bg-red-600 rounded-full">
                    {"0"}
                </span>
            </a>
        </nav>
    }
}
