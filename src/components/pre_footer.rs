use leptos::prelude::*;

use crate::components::video::VideoPlayer;

#[component]
pub fn BasePreFooter() -> impl IntoView {
    view! {
        <section class="py-16 text-white bg-gray-900">
            <div class="grid grid-cols-1 gap-12 px-4 mx-auto max-w-6xl md:grid-cols-3">

                <div class="flex flex-col space-y-8">
                    <div>
                        <VideoPlayer
                            src="assets/videos/show_01.mp4"
                            autoplay=true
                            controls=true
                            loop_=true
                            muted=true
                        />
                    </div>
                </div>

                <div class="md:col-span-2">
                    <div class="p-8 bg-gray-800 rounded-lg shadow-lg">
                        <p class="mb-2 text-lg">
                            Es el curso que estas buscando
                            <span class="font-semibold text-blue-400">tenemos</span>tambien
                            <span class="font-semibold text-blue-400">Más Productos</span>?
                        </p>
                        <p class="mb-6 text-sm text-gray-300">
                            No te preocupes. Inscríbete en la lista de espera de DentLit Academy y, en cuanto se abra un
                            cupo o un nuevo curso, te lo haremos saber.
                        </p>

                        <form class="space-y-4">
                            <input
                                type="text"
                                placeholder="Nombre completo*"
                                required
                                class="py-3 px-4 w-full text-gray-900 rounded-md border border-gray-300 outline-none focus:ring-2 focus:ring-blue-500"
                            />
                            <input
                                type="tel"
                                placeholder="Numero de telefono*"
                                required
                                class="py-3 px-4 w-full text-gray-900 rounded-md border border-gray-300 outline-none focus:ring-2 focus:ring-blue-500"
                            />
                            <input
                                type="email"
                                placeholder="Email*"
                                required
                                class="py-3 px-4 w-full text-gray-900 rounded-md border border-gray-300 outline-none focus:ring-2 focus:ring-blue-500"
                            />

                            <select
                                required
                                class="py-3 px-4 w-full text-gray-900 rounded-md border border-gray-300 outline-none focus:ring-2 focus:ring-blue-500"
                            >
                                <option value="">
                                    {"¿Cual es el evento que desea asistir?*"}
                                </option>
                                <option value="Dental Photography Courses">
                                    Dental Photography Courses
                                </option>
                                <option value="Adhesive Rehabilitation and Tooth Preparation Courses">
                                    Adhesive Rehabilitation and Tooth Preparation Courses
                                </option>
                                <option value="Booking Request">Booking Request</option>
                                <option value="Dental Photography Products">
                                    Dental Photography Products
                                </option>
                                <option value="Shade Taking Instruments">
                                    Shade Taking Instruments
                                </option>
                                <option value="Dental Burs">Dental Burs</option>
                                <option value="General Inquiry">General Inquiry</option>
                                <option value="Other">Other</option>
                            </select>

                            <label class="flex items-start space-x-2 text-xs text-gray-400">
                                <input type="checkbox" required class="mt-1" />
                                <span>
                                    Al enviar tu información, aceptas recibir correos electrónicos y mensajes de texto
                                    con información de marketing de nuestra parte. Puedes darte de baja en cualquier
                                    momento.
                                </span>
                            </label>

                            <button
                                type="submit"
                                class="py-3 w-full font-medium bg-blue-600 rounded-md transition hover:bg-blue-700"
                            >
                                Sign Up
                            </button>
                        </form>
                    </div>
                    <div class="mt-12">
                        <ul class="flex space-x-4">
                            <li>
                                <a href="/" target="_blank" class="">
                                    <img
                                        src="assets/icons/facebook.svg"
                                        alt="Facebook"
                                        class="w-8 h-8 filter invert brightness-0"
                                    />
                                </a>
                            </li>
                            <li>
                                <a href="/" target="_blank" class="">
                                    <img
                                        src="/assets/icons/instagram.svg"
                                        alt="Instagram"
                                        class="w-8 h-8 filter invert brightness-0"
                                    />
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}
