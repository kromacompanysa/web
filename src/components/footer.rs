use leptos::prelude::*;

#[component]
pub fn BaseFooter() -> impl IntoView {
    view! {
        <footer class="text-gray-400 border-t border-gray-800 bg-gray-950">
            <div class="py-8 px-6 mx-auto max-w-6xl text-sm text-center">

                // <!-- Social Icons -->
                <ul class="flex justify-center mb-3 space-x-5">
                    <li>
                        <a
                            href="https://facebook.com"
                            target="_blank"
                            aria-label="Facebook"
                            class="transition hover:opacity-80"
                        >
                            <img
                                src="assets/icons/facebook.svg"
                                alt="Facebook"
                                class="w-6 h-6 invert brightness-0"
                            />
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://instagram.com"
                            target="_blank"
                            aria-label="Instagram"
                            class="transition hover:opacity-80"
                        >
                            <img
                                src="assets/icons/instagram.svg"
                                alt="Instagram"
                                class="w-6 h-6 invert brightness-0"
                            />
                        </a>
                    </li>
                </ul>
                <p class="mb-2 space-x-2">
                    <a href="/policies/terms-of-service" class="transition hover:text-white">
                        {"Terminos de Servicio"}
                    </a>
                    <span>|</span>
                    <a href="/policies/refund-policy" class="transition hover:text-white">
                        {"Política de Reembolso"}
                    </a>
                    <span>|</span>
                    <a href="https://docs.cordstart.com/projects/7/" target="_blank" class="transition hover:text-white">
                      {"Desarrollado por CordStart Tech"}
                    </a>
                </p>
                <p class="text-gray-500">
                    {"© 2025, Dr. Kroma Company S.A. Todos los derechos reservados."}
                </p>
            </div>
        </footer>
    }
}
