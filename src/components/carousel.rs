use leptos::prelude::*;
use leptos_router::hooks::use_location;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = initHeroSwiper)]
    fn init_hero_swiper();
}

#[component]
pub fn BaseCarousel() -> impl IntoView {
    let root_ref = NodeRef::<leptos::html::Div>::new();
    let location = use_location();

    // Every time user navigates → re-run JS
    Effect::new({
        let root_ref = root_ref.clone();
        move |_| {
            let _ = location.pathname.get();

            if root_ref.get().is_none() {
                return;
            }

            init_hero_swiper();
        }
    });

    view! {
        <section class="pt-20 pb-10 bg-gradient-to-b from-gray-50 to-white">
            <div class="px-4 mx-auto max-w-6xl">
                // <!-- Section Title -->
                <div class="mb-12 text-center">
                    <h2 class="text-3xl font-extrabold tracking-tight text-gray-900 md:text-4xl">
                        {"Elige el camino en "}
                        <span class="text-transparent bg-clip-text bg-gradient-to-r from-red-500 to-red-700">
                            {"Kroma Academy "}
                        </span> {"que mejor se adapte a tu estilo de aprendizaje."}
                    </h2>
                    <p class="mx-auto mt-4 max-w-2xl text-gray-600">
                        {"Explora nuestros cursos y experiencias diseñadas para profesionales de la odontología moderna."}
                    </p>
                </div>

                // <!-- Swiper -->
                <div node_ref=root_ref class="swiper mySwiperHero hero-swiper">
                    <div class="swiper-wrapper">
                        {([
                            (
                                "Curso Tema 1",
                                "//dentlit.com/cdn/shop/files/dentlit-online-course-thumbnail_2x_103da2ba-7fdc-4b58-ba35-39ceec0b05b1_160x160@2x.png?v=1675373315",
                                "/pages/curso/1",
                            ),
                            (
                                "Curso Tema 2",
                                "//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337",
                                "/pages/curso/2",
                            ),
                            (
                                "Curso Tema 3",
                                "//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353",
                                "/pages/curso/3",
                            ),
                            (
                                "Curso Tema 4",
                                "//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337",
                                "/pages/curso/4",
                            ),
                            (
                                "Curso Tema 5",
                                "//dentlit.com/cdn/shop/files/dentlit-life-in-person-thumbnail_2x_b695378b-3051-4189-84a3-97c0947a3908_160x160@2x.png?v=1675373337",
                                "/pages/curso/5",
                            ),
                        ])
                            .into_iter()
                            .map(|(title, img, url)| {
                                view! {
                                    <div class="swiper-slide">
                                        <a
                                            href={url}
                                            class="block overflow-hidden relative w-full h-72 rounded-3xl shadow-md transition-all duration-500 hover:shadow-xl hover:-translate-y-1 group"
                                        >
                                            <img
                                                src={img}
                                                alt={title}
                                                class="object-cover w-full h-full transition-transform duration-700 transform scale-105 group-hover:scale-110"
                                            />
                                            <div class="absolute inset-0 bg-gradient-to-t to-transparent opacity-90 transition-opacity group-hover:opacity-95 from-black/70 via-black/30"></div>
                                            <span class="absolute bottom-6 left-1/2 py-4 px-10 text-base font-semibold text-center text-white bg-red-600 rounded-full shadow-md transition duration-300 transform -translate-x-1/2 hover:bg-red-700 hover:shadow-lg hover:scale-105">
                                                {title}
                                            </span>
                                        </a>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>

                    // <!-- Swiper pagination -->
                    <div class="mt-10 swiper-pagination"></div>
                </div>
            </div>
        </section>
    }
}
