use leptos::prelude::*;

use crate::components::nav::BaseNav;

#[component]
pub fn BaseHeader() -> impl IntoView {
    // mobile menu state
    let (open, set_open) = signal(false);

    view! {
        <header class="fixed top-0 z-50 w-full shadow-sm transition-all duration-300 bg-white/90 backdrop-blur-md">
            <div class="flex justify-between items-center py-3 px-6 mx-auto max-w-7xl md:py-4">
                // Logo
                <a href="/" class="flex items-center space-x-2 group">
                    <img
                        src="assets/images/kroma_png_black.png"
                        alt="Kroma Logo"
                        class="w-28 h-auto transition-transform duration-300 sm:w-36 md:w-40 group-hover:scale-105"
                    />
                </a>

                // Desktop nav
                <BaseNav />

                // Mobile button
                <button
                    class="flex items-center text-gray-700 md:hidden focus:outline-none"
                    on:click={move |_| set_open.update(|o| *o = !*o)}
                    aria-label="Toggle menu"
                    aria-expanded={move || if open.get() { "true" } else { "false" }}
                >
                    <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16"
                        />
                    </svg>
                </button>
            </div>

            // Mobile menu (reuses the style system)
            <div class={move || {
                format!(
                    "{} {}",
                    "md:hidden border-t border-gray-200 bg-white",
                    if open.get() { "block" } else { "hidden" },
                )
            }}>
                <ul class="flex flex-col p-4 space-y-2 font-medium text-gray-700">
                    <li>
                        <a
                            href="/about"
                            class="block py-2 px-4 rounded-md transition hover:text-blue-600 hover:bg-gray-100"
                        >
                            {"¿Quiénes somos?"}
                        </a>
                    </li>

                    {}
                    <li class="pt-2 border-t border-gray-200">
                        <details class="group">
                            <summary class="flex justify-between items-center py-2 px-4 rounded-md transition cursor-pointer hover:text-blue-600 hover:bg-gray-100">
                                {"Clínica dental"}
                                <svg
                                    class="ml-2 w-4 h-4 transition-transform duration-200 group-open:rotate-180"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.2l3.71-3.97a.75.75 0 111.08 1.04l-4.25 4.55a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z"
                                        clip-rule="evenodd"
                                    />
                                </svg>
                            </summary>

                            <ul class="py-2 pr-2 pl-6 space-y-1 bg-gray-50 rounded-md">
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Periodoncia"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Endodoncia"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Ortodoncia"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Estética"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Implantología"}
                                    </a>
                                </li>
                            </ul>
                        </details>
                    </li>

                    {}
                    <li class="pt-2 border-t border-gray-200">
                        <details class="group">
                            <summary class="flex justify-between items-center py-2 px-4 rounded-md transition cursor-pointer hover:text-blue-600 hover:bg-gray-100">
                                {"Academia"}
                                <svg
                                    class="ml-2 w-4 h-4 transition-transform duration-200 group-open:rotate-180"
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.2l3.71-3.97a.75.75 0 111.08 1.04l-4.25 4.55a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z"
                                        clip-rule="evenodd"
                                    />
                                </svg>
                            </summary>

                            <ul class="py-2 pr-2 pl-6 space-y-1 bg-gray-50 rounded-md">
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Fotografía odontológica"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Rehabilitación oral"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Operatoria"}
                                    </a>
                                </li>
                                <li>
                                    <a
                                        href="/"
                                        class="block py-1 px-2 rounded transition hover:text-blue-600 hover:bg-gray-100"
                                    >
                                        {"Resinas estratificadas"}
                                    </a>
                                </li>
                            </ul>
                        </details>
                    </li>

                    <li class="pt-2 border-t border-gray-200">
                        <a
                            href="/contact"
                            class="block py-2 px-4 rounded-md transition hover:text-blue-600 hover:bg-gray-100"
                        >
                            {"Contacto"}
                        </a>
                    </li>

                    <li class="pt-2 border-t border-gray-200">
                        <a
                            href="/cart"
                            class="flex gap-2 items-center py-2 px-4 rounded-md transition hover:text-blue-600 hover:bg-gray-100"
                        >
                            <img src="assets/icons/cart4.svg" alt="Cart" class="w-5 h-5" />
                            <span>{"Tienda"}</span>

                            // <!-- aligned badge -->
                            <span class="py-0.5 px-1.5 ml-auto text-white bg-red-600 rounded-full text-[10px]">
                                {"0"}
                            </span>
                        </a>
                    </li>
                </ul>
            </div>
        </header>
    }
}
