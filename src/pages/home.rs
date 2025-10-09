use leptos::prelude::*;

use crate::components::carousel::BaseCarousel;
use crate::components::hero::BaseHero;

#[component]
pub fn Home() -> impl IntoView {
    view! {

        <BaseHero/>

        <section class="py-16 bg-gray-50">
          <div class="max-w-5xl mx-auto px-4">

            <div class="text-center mb-12">
              <h2 class="text-2xl md:text-3xl font-bold text-gray-900">
                Elige el camino en Kroma Academy que mejor se adapte a tu estilo de aprendizaje.
              </h2>
            </div>

            <div class="relative">
              <div class="flex overflow-x-auto snap-x snap-mandatory gap-6 pb-4 scroll-smooth">
                <a
                  href="/pages/online-courses-and-free-content"
                  class="relative flex-none w-72 h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition snap-center"
                >
                  <img
                    src="//dentlit.com/cdn/shop/files/dentlit-online-course-thumbnail_2x_103da2ba-7fdc-4b58-ba35-39ceec0b05b1_160x160@2x.png?v=1675373315"
                    alt="Online Courses"
                    class="absolute inset-0 w-full h-full object-cover"
                  />
                  <span
                    class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                  >
                    Curso <br /> Tema 1
                  </span>
                </a>

                <a
                  href="/pages/live-in-person-experiences"
                  class="relative flex-none w-72 h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition snap-center"
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

                <a
                  href="/pages/books-and-products"
                  class="relative flex-none w-72 h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition snap-center"
                >
                  <img
                    src="//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353"
                    alt="Books and Products"
                    class="absolute inset-0 w-full h-full object-cover"
                  />
                  <span
                    class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                  >
                    Curso <br /> Tema 3
                  </span>
                </a>
                <a
                  href="/pages/books-and-products"
                  class="relative flex-none w-72 h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition snap-center"
                >
                  <img
                    src="//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353"
                    alt="Books and Products"
                    class="absolute inset-0 w-full h-full object-cover"
                  />
                  <span
                    class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                  >
                    Curso <br /> Tema 3
                  </span>
                </a>
                <a
                  href="/pages/books-and-products"
                  class="relative flex-none w-72 h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition snap-center"
                >
                  <img
                    src="//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353"
                    alt="Books and Products"
                    class="absolute inset-0 w-full h-full object-cover"
                  />
                  <span
                    class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                  >
                    Curso <br /> Tema 3
                  </span>
                </a>
                <a
                  href="/pages/books-and-products"
                  class="relative flex-none w-72 h-64 rounded-2xl overflow-hidden shadow-md hover:shadow-lg transition snap-center"
                >
                  <img
                    src="//dentlit.com/cdn/shop/files/books-and-products-thumbnail_2x_8c716495-022a-4fd4-8681-2b4298ac41a9_160x160@2x.png?v=1675373353"
                    alt="Books and Products"
                    class="absolute inset-0 w-full h-full object-cover"
                  />
                  <span
                    class="absolute bottom-4 left-1/2 -translate-x-1/2 text-lg font-medium text-white bg-black/50 px-4 py-2 rounded-lg"
                  >
                    Curso <br /> Tema 3
                  </span>
                </a>

              </div>
            </div>
          </div>
        </section>

        <BaseCarousel/>


        <section class="py-16 bg-gray-50">
            <div class="w-full px-4">
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-6">

                <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 1
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 2
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 3
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 4
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 5
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 6
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 7
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 8
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 9
                    </div>

                    <div class="h-32 bg-gray-200 rounded-lg flex items-center justify-center text-gray-600 font-medium">
                        Box 10
                    </div>
                </div>
            </div>
        </section>
    }
}
