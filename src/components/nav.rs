use leptos::prelude::*;

use crate::components::nav_dropdown::HeaderDropdownMenu;

#[component]
pub fn BaseNav() -> impl IntoView {
    view! {
        <nav class="hidden md:flex space-x-8 items-center font-medium text-gray-700">
            <a href="/about" class="hover:text-blue-600 transition">{"¿Quiénes somos?"}</a>

            <HeaderDropdownMenu
                title="Clinica dental"
                items=&[
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
                ]
            />

            <HeaderDropdownMenu
                title="Clinica dental"
                items=&[
                    ("/", "Fotografía odontológica"),
                    ("/", "Protesis completa"),
                    ("/", "Resinas con guía transparente"),
                    ("/", "Operatoria"),
                    ("/", "Rehabilitación oral"),
                    ("/", "Resinas estratificadas"),
                    ("/", "Endodoncia"),
                    ("/", "Periodoncia e implantes"),
                    ("/", "Ortodoncia"),
                    ("/", "Odontológía digital"),
                ]
            />

            <a href="/contact" class="hover:text-blue-600 transition">Contacto</a>

            <a href="/cart" class="relative flex items-center hover:text-blue-600">
                <div class="w-4 h-4 text-black-600">
                    <img src="assets/icons/cart4.svg" />
                </div>
            </a>
        </nav>
    }
}
