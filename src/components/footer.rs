use leptos::prelude::*;

#[component]
pub fn BaseFooter() -> impl IntoView {
    view! {
        <footer class="bg-gray-900 text-white py-16">
            <div class="max-w-6xl mx-auto px-4 grid grid-cols-1 md:grid-cols-3 gap-12">

                <div class="flex flex-col space-y-8">
                    <div>
                        <h4 class="text-lg font-semibold mb-4">Mantente conectado</h4>
                        <form class="flex flex-col sm:flex-row gap-3">
                            <input
                                type="email"
                                placeholder="Subscribete a nuestro canal"
                                required
                                class="flex-1 px-4 py-2 rounded-md text-gray-900 focus:ring-2 focus:ring-blue-500 outline-none"
                            />
                            <button
                                type="submit"
                                class="bg-blue-600 hover:bg-blue-700 px-6 py-2 rounded-md font-medium transition"
                            >
                                Sign Up
                            </button>
                        </form>
                    </div>

                    <div>
                        <ul class="flex space-x-4">
                            <li>
                                <a
                                    href="https://www.facebook.com/BestofProsth/"
                                    target="_blank"
                                    class="text-gray-400 hover:text-white transition"
                                >
                                    <i class="fab fa-facebook-f text-xl"></i>
                                    <span class="sr-only">Facebook</span>
                                </a>
                            </li>
                            <li>
                                <a
                                    href="http://instagram.com/dr_miguel_ortiz/"
                                    target="_blank"
                                    class="text-gray-400 hover:text-white transition"
                                >
                                    <i class="fab fa-instagram text-xl"></i>
                                    <span class="sr-only">Instagram</span>
                                </a>
                            </li>
                        </ul>
                    </div>

                    <div>
                        <h4 class="text-lg font-semibold mb-4">Contactarse ahora con Dr. Jujuy</h4>
                        <a
                            href="https://dentlit.com/pages/contact"
                            class="inline-block bg-blue-600 hover:bg-blue-700 px-6 py-3 rounded-md font-medium transition"
                        >
                            Contactarse ahora con Dr. Jujuy.
                        </a>
                    </div>
                </div>

                <div class="md:col-span-2">
                    <div class="bg-gray-800 p-8 rounded-lg shadow-lg">
                        <p class="text-lg mb-2">
                            Es el curso que estas buscando<span class="font-semibold text-blue-400"> tenemos</span>
                            tambien <span class="font-semibold text-blue-400">Más Productos</span>?
                        </p>
                        <p class="text-sm text-gray-300 mb-6">
                            No te preocupes. Inscríbete en la lista de espera de DentLit Academy y, en cuanto se abra un
                            cupo o un nuevo curso, te lo haremos saber.
                        </p>

                        <form class="space-y-4">
                            <input
                                type="text"
                                placeholder="Nombre completo*"
                                required
                                class="w-full px-4 py-3 rounded-md text-gray-900 border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
                            />
                            <input
                                type="tel"
                                placeholder="Numero de telefono*"
                                required
                                class="w-full px-4 py-3 rounded-md text-gray-900 border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
                            />
                            <input
                                type="email"
                                placeholder="Email*"
                                required
                                class="w-full px-4 py-3 rounded-md text-gray-900 border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
                            />

                            <select
                                required
                                class="w-full px-4 py-3 rounded-md text-gray-900 border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none"
                            >
                                <option value="">{"¿Cual es el evento que desea asistir?*"}</option>
                                <option value="Dental Photography Courses">Dental Photography Courses</option>
                                <option value="Adhesive Rehabilitation and Tooth Preparation Courses">
                                    Adhesive Rehabilitation and Tooth Preparation Courses
                                </option>
                                <option value="Booking Request">Booking Request</option>
                                <option value="Dental Photography Products">Dental Photography Products</option>
                                <option value="Shade Taking Instruments">Shade Taking Instruments</option>
                                <option value="Dental Burs">Dental Burs</option>
                                <option value="General Inquiry">General Inquiry</option>
                                <option value="Other">Other</option>
                            </select>

                            <label class="flex items-start text-xs text-gray-400 space-x-2">
                                <input type="checkbox" required class="mt-1" />
                                <span
                                    >Al enviar tu información, aceptas recibir correos electrónicos y mensajes de texto
                                    con información de marketing de nuestra parte. Puedes darte de baja en cualquier
                                    momento.</span
                                >
                            </label>

                            <button
                                type="submit"
                                class="w-full bg-blue-600 hover:bg-blue-700 py-3 rounded-md font-medium transition"
                            >
                                Sign Up
                            </button>
                        </form>
                    </div>
                </div>
            </div>

            <div class="border-t border-gray-700 mt-12 pt-6 text-center text-sm text-gray-400">
                <p class="mb-2">
                    <a href="/policies/terms-of-service" class="hover:text-white">Terms of service</a> |
                    <a href="/policies/refund-policy" class="hover:text-white">Refund policy</a>
                </p>
                <p>{"© 2025, Dr. Jaime Ramos. All Rights Reserved"}</p>
            </div>
        </footer>
    }
}
