use leptos::prelude::*;

use crate::components::nav::BaseNav;

#[component]
pub fn BaseHeader() -> impl IntoView {
    view! {
        <header class="fixed top-0 z-50 w-full bg-white shadow-md">
            <div class="flex justify-between items-center py-3 px-4 mx-auto max-w-7xl md:py-4">
                <a href="/" class="flex items-center">
                    <img
                        src="assets/images/kroma_png_black.png"
                        alt="Kroma Logo"
                        class="w-24 h-auto sm:w-32 md:w-40 lg:w-48 xl:w-56"
                    />
                </a>

                <BaseNav />

                <button
                    id="menu-btn"
                    class="flex items-center text-gray-700 md:hidden focus:outline-none"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16"
                        />
                    </svg>
                </button>
            </div>

            <div id="mobile-menu" class="hidden bg-white border-t border-gray-200 md:hidden">
                <ul class="flex flex-col p-4 space-y-2">
                    <li>
                        <a href="/pages/about" class="block hover:text-blue-600">
                            About
                        </a>
                    </li>
                    <li>
                        <a
                            href="/pages/live-in-person-experiences"
                            class="block hover:text-blue-600"
                        >
                            Live in Person Experiences
                        </a>
                    </li>
                    <li>
                        <a
                            href="/pages/online-courses-and-free-content"
                            class="block hover:text-blue-600"
                        >
                            Online Courses and Content
                        </a>
                    </li>
                    <li>
                        <a href="/pages/cursos-en-espanol" class="block hover:text-blue-600">
                            Cursos en Español
                        </a>
                    </li>
                    <li>
                        <a href="/pages/books-and-products" class="block hover:text-blue-600">
                            Books & Products
                        </a>
                    </li>
                    <li>
                        <a href="/pages/contact" class="block hover:text-blue-600">
                            Contact
                        </a>
                    </li>
                </ul>
            </div>
        </header>
    }
}
