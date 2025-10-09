use leptos::prelude::*;

#[component]
pub fn BaseHero() -> impl IntoView {
    view! {
        <section
          class="relative flex items-center justify-center text-center pt-32 min-h-screen bg-[url('assets/images/hero.jpeg')] bg-cover bg-center bg-no-repeat"
        >
          <div class="absolute inset-0 bg-black/40"></div>

          <div class="relative max-w-2xl mx-auto px-4 text-white">
              <ul class="space-y-6">
                  <li>
                      <span class="block text-md uppercase tracking-widest text-blue-300 font-semibold">
                          {"Bienvenido a Koma"}
                      </span>
                      <h1 class="mt-2 text-3xl md:text-5xl font-bold leading-tight">
                          {"Donde la sonrisa se convierte en arte."}
                      </h1>
                  </li>
                  <li>
                      <h2 class="text-lg md:text-xl font-medium">
                          {"Un enfoque moderno de la odontología basada en la evidencia."}<br />
                          {"Fundada por la Dra. Kelly y Dr. Jujuy"}
                      </h2>
                  </li>
              </ul>
          </div>
        </section>
    }
}
