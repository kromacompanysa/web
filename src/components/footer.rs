use leptos::prelude::*;

#[component]
pub fn BaseFooter() -> impl IntoView {
    view! {
        <footer class="text-gray-400 border-t border-gray-800 bg-gray-950">
            <div class="py-8 px-6 mx-auto max-w-6xl text-sm text-center">
                <p class="mb-2 space-x-2">
                    <a href="/policies/terms-of-service" class="transition hover:text-white">
                        {"Terminos de Servicio"}
                    </a>
                    <span>|</span>
                    <a href="/policies/refund-policy" class="transition hover:text-white">
                        {"Política de Reembolso"}
                    </a>
                </p>
                <p class="text-gray-500">
                    {"© 2025, Dr. Kroma Company S.A. Todos los derechos reservados."}
                </p>
            </div>
        </footer>
    }
}
