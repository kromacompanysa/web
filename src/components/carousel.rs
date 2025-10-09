use leptos::prelude::*;

#[component]
pub fn BaseCarousel() -> impl IntoView {
    // ✅ Return the view
    view! {
        <section class="py-16 bg-gray-50">
            <div class="max-w-5xl mx-auto px-4">
                <div class="swiper mySwiper max-w-5xl mx-auto px-4">
                    <div class="text-center mb-12">
                        <h2 class="text-2xl md:text-3xl font-bold text-gray-900">
                            {"Elige el camino en Kroma Academy que mejor se adapte a tu estilo de aprendizaje."}
                        </h2>
                    </div>
                    <div class="swiper-wrapper">
                        <div class="swiper-slide">
                            <a
                            href="/pages/live-in-person-experiences"
                            class="relative block w-full h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition"
                            >
                            <img
                                src="//dentlit.com/cdn/shop/files/dentlit-online-course-thumbnail_2x_103da2ba-7fdc-4b58-ba35-39ceec0b05b1_160x160@2x.png?v=1675373315"
                                alt="Live In Person Experiences"
                                class="absolute inset-0 w-full h-full object-cover"
                            />
                            <span
                                class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                            >
                                Curso <br /> Tema 1
                            </span>
                            </a>
                        </div>
                        <div class="swiper-slide">
                            <a
                            href="/pages/live-in-person-experiences"
                            class="relative block w-full h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition"
                            >
                            <img
                                src="//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337"
                                alt="Live In Person Experiences"
                                class="absolute inset-0 w-full h-full object-cover"
                            />
                            <span
                                class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                            >
                                Curso <br /> Tema 2
                            </span>
                            </a>
                        </div>
                        <div class="swiper-slide">
                            <a
                            href="/pages/live-in-person-experiences"
                            class="relative block w-full h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition"
                            >
                            <img
                                src="//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353"
                                alt="Live In Person Experiences"
                                class="absolute inset-0 w-full h-full object-cover"
                            />
                            <span
                                class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                            >
                                Curso <br /> Tema 3
                            </span>
                            </a>
                        </div>
                        <div class="swiper-slide">
                            <a
                            href="/pages/live-in-person-experiences"
                            class="relative block w-full h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition"
                            >
                            <img
                                src="//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337"
                                alt="Live In Person Experiences"
                                class="absolute inset-0 w-full h-full object-cover"
                            />
                            <span
                                class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                            >
                                Curso <br /> Tema 4
                            </span>
                            </a>
                        </div>
                        <div class="swiper-slide">
                            <a
                            href="/pages/live-in-person-experiences"
                            class="relative block w-full h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition"
                            >
                            <img
                                src="//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337"
                                alt="Live In Person Experiences"
                                class="absolute inset-0 w-full h-full object-cover"
                            />
                            <span
                                class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                            >
                                Curso <br /> Tema 5
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
