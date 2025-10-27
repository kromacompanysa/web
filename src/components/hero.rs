use leptos::prelude::*;

#[component]
pub fn BaseHero() -> impl IntoView {
    view! {
        <section class="flex relative justify-center items-center pt-32 min-h-screen text-center bg-center bg-no-repeat bg-cover bg-[url('assets/images/hero.jpeg')]">
            // <!-- Gradient overlay for depth and contrast -->
            <div class="absolute inset-0 bg-gradient-to-b from-black/70 via-black/60 to-black/80"></div>

            <div class="relative z-10 px-6 mx-auto max-w-3xl text-white">
                <ul class="space-y-10">
                    // <!-- Tagline -->
                    <li>
                        <span class="inline-block mb-2 text-sm font-semibold text-red-500 uppercase md:text-base tracking-[0.25em] drop-shadow-[0_0_8px_rgba(255,0,0,0.4)]">
                            {"Bienvenido a Kroma"}
                        </span>

                        // <!-- Main title -->
                        <h1 class="text-5xl font-extrabold leading-tight text-white md:text-7xl drop-shadow-[0_3px_12px_rgba(0,0,0,0.8)]">
                            {"Donde la sonrisa"} <br class="hidden md:block" /> {"se convierte en"}
                            <span class="text-red-600">{" arte."}</span>
                        </h1>
                    </li>

                    // <!-- Subtitle -->
                    <li>
                        <h2 class="text-lg font-light leading-relaxed text-gray-200 md:text-xl">
                            {"Un enfoque moderno de la odontología basada en la evidencia."}<br />
                            <span class="font-medium text-gray-100">
                                {"Fundada por la Dra. Kelly Luna y Dr. Jaime Ramos"}
                            </span>
                        </h2>
                    </li>

                    // <!-- Call to Action -->
                    <li>
                        <a
                            href="#contacto"
                            class="inline-block py-4 px-10 mt-2 text-base font-semibold text-white bg-red-600 rounded-full shadow-md transition duration-300 transform hover:bg-red-700 hover:shadow-lg hover:scale-105"
                        >
                            {"Reserva tu cita"}
                        </a>
                    </li>
                </ul>
            </div>
        </section>
    }
}
