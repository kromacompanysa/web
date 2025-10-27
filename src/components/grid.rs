use leptos::prelude::*;

#[component]
pub fn BaseGrid() -> impl IntoView {
    view! {
        <section class="pt-10 pb-10 bg-gray-50">
            <div class="swiper mySwiperBoxes boxes-swiper">
                // <!-- Section Title -->
                <div class="mb-12 text-center">
                    <h2 class="text-3xl font-extrabold tracking-tight text-gray-900 md:text-4xl">
                        {"Casos de éxito."}
                    </h2>
                    <p class="mx-auto mt-4 max-w-2xl text-gray-600">
                        {"Resultados reales de pacientes que confiaron en Kroma Dental Studio."}
                    </p>
                </div>

                // <!-- Swiper Wrapper -->
                <div class="swiper-wrapper">
                    {([
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                        "assets/images/grid_01.jpeg",
                    ])
                        .into_iter()
                        .map(|img| {
                            // add more as needed
                            view! {
                                <div class="swiper-slide boxes-slide">
                                    <div class="overflow-hidden relative h-64">
                                        <img
                                            src={img}
                                            alt="Caso de éxito Kroma"
                                            class="object-cover absolute inset-0 w-full h-full"
                                        />
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                // <!-- Pagination -->
                <div class="mt-8 swiper-pagination"></div>
            </div>
        </section>
    }
}
