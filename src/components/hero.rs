use leptos::prelude::*;

#[component]
pub fn BaseHero() -> impl IntoView {
    view! {
        <section class="flex relative justify-center items-center pt-32 min-h-screen text-center bg-center bg-no-repeat bg-cover bg-[url('assets/images/hero.jpeg')]">
            <div class="absolute inset-0 bg-black/40"></div>

            <div class="relative px-4 mx-auto max-w-2xl text-white">
                <ul class="space-y-6">
                    <li>
                        <span class="block font-semibold tracking-widest text-blue-300 uppercase text-md">
                            {"Bienvenido a Koma"}
                        </span>
                        <h1 class="mt-2 text-3xl font-bold leading-tight md:text-5xl">
                            {"Donde la sonrisa se convierte en arte."}
                        </h1>
                    </li>
                    <li>
                        <h2 class="text-lg font-medium md:text-xl">
                            {"Un enfoque moderno de la odontología basada en la evidencia."}<br />
                            {"Fundada por la Dra. Kelly y Dr. Jujuy"}
                        </h2>
                    </li>
                </ul>
            </div>
        </section>
    }
}
