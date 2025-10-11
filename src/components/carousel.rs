use leptos::prelude::*;

#[component]
pub fn BaseCarousel() -> impl IntoView {
    // ✅ Return the view
    view! {
        <section class="pt-16 bg-gray-50">
            <div class="px-4 mx-auto max-w-5xl">
                <div class="px-4 mx-auto max-w-5xl swiper mySwiperHero hero-swiper">
                    <div class="mb-12 text-center">
                        <h2 class="text-2xl font-bold text-gray-900 md:text-3xl">
                            {"Elige el camino en Kroma Academy que mejor se adapte a tu estilo de aprendizaje."}
                        </h2>
                    </div>
                    <div class="swiper-wrapper">
                        <div class="swiper-slide hero-slide">
                            <a
                                href="/pages/live-in-person-experiences"
                                class="block overflow-hidden relative w-full h-64 rounded-2xl shadow-md transition hover:shadow-lg"
                            >
                                <img
                                    src="//dentlit.com/cdn/shop/files/dentlit-online-course-thumbnail_2x_103da2ba-7fdc-4b58-ba35-39ceec0b05b1_160x160@2x.png?v=1675373315"
                                    alt="Live In Person Experiences"
                                    class="object-cover absolute inset-0 w-full h-full"
                                />
                                <span class="absolute bottom-4 left-1/2 py-2 px-4 text-lg font-medium text-white rounded-lg -translate-x-1/2 bg-black/50">
                                    Curso <br />Tema 1
                                </span>
                            </a>
                        </div>
                        <div class="swiper-slide hero-slide">
                            <a
                                href="/pages/live-in-person-experiences"
                                class="block overflow-hidden relative w-full h-64 rounded-2xl shadow-md transition hover:shadow-lg"
                            >
                                <img
                                    src="//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337"
                                    alt="Live In Person Experiences"
                                    class="object-cover absolute inset-0 w-full h-full"
                                />
                                <span class="absolute bottom-4 left-1/2 py-2 px-4 text-lg font-medium text-white rounded-lg -translate-x-1/2 bg-black/50">
                                    Curso <br />Tema 2
                                </span>
                            </a>
                        </div>
                        <div class="swiper-slide hero-slide">
                            <a
                                href="/pages/live-in-person-experiences"
                                class="block overflow-hidden relative w-full h-64 rounded-2xl shadow-md transition hover:shadow-lg"
                            >
                                <img
                                    src="//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353"
                                    alt="Live In Person Experiences"
                                    class="object-cover absolute inset-0 w-full h-full"
                                />
                                <span class="absolute bottom-4 left-1/2 py-2 px-4 text-lg font-medium text-white rounded-lg -translate-x-1/2 bg-black/50">
                                    Curso <br />Tema 3
                                </span>
                            </a>
                        </div>
                        <div class="swiper-slide hero-slide">
                            <a
                                href="/pages/live-in-person-experiences"
                                class="block overflow-hidden relative w-full h-64 rounded-2xl shadow-md transition hover:shadow-lg"
                            >
                                <img
                                    src="//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337"
                                    alt="Live In Person Experiences"
                                    class="object-cover absolute inset-0 w-full h-full"
                                />
                                <span class="absolute bottom-4 left-1/2 py-2 px-4 text-lg font-medium text-white rounded-lg -translate-x-1/2 bg-black/50">
                                    Curso <br />Tema 4
                                </span>
                            </a>
                        </div>
                        <div class="swiper-slide hero-slide">
                            <a
                                href="/pages/live-in-person-experiences"
                                class="block overflow-hidden relative w-full h-64 rounded-2xl shadow-md transition hover:shadow-lg"
                            >
                                <img
                                    src="//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337"
                                    alt="Live In Person Experiences"
                                    class="object-cover absolute inset-0 w-full h-full"
                                />
                                <span class="absolute bottom-4 left-1/2 py-2 px-4 text-lg font-medium text-white rounded-lg -translate-x-1/2 bg-black/50">
                                    Curso <br />Tema 5
                                </span>
                            </a>
                        </div>
                    </div>
                    <div class="swiper-pagination"></div>
                </div>
            </div>
        </section>
    }
}
