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
                            {"Es el curso que estas buscando"}
                            <span class="font-semibold text-blue-400">{"tenemos"}</span>tambien
                            <span class="font-semibold text-blue-400">{"Más Productos"}</span>?
                        </p>
                        <p class="mb-6 text-sm text-gray-300">
                            {"No te preocupes. Inscríbete en la lista de espera de DentLit Academy y, en cuanto se abra un
                            cupo o un nuevo curso, te lo haremos saber."}
                        </p>

                        <form class="mx-auto space-y-5 max-w-lg">
                            // <!-- Nombre -->
                            <div>
                                <input
                                    type="text"
                                    name="name"
                                    placeholder="Nombre completo*"
                                    required
                                    class="py-3 px-4 w-full placeholder-gray-500 text-gray-900 rounded-lg border border-gray-300 transition focus:border-red-500 focus:ring-2 focus:ring-red-500 focus:outline-none"
                                />
                            </div>

                            // <!-- Teléfono -->
                            <div>
                                <input
                                    type="tel"
                                    name="phone"
                                    placeholder="Número de teléfono*"
                                    required
                                    class="py-3 px-4 w-full placeholder-gray-500 text-gray-900 rounded-lg border border-gray-300 transition focus:border-red-500 focus:ring-2 focus:ring-red-500 focus:outline-none"
                                />
                            </div>

                            // <!-- Email -->
                            <div>
                                <input
                                    type="email"
                                    name="email"
                                    placeholder="Correo electrónico*"
                                    required
                                    class="py-3 px-4 w-full placeholder-gray-500 text-gray-900 rounded-lg border border-gray-300 transition focus:border-red-500 focus:ring-2 focus:ring-red-500 focus:outline-none"
                                />
                            </div>

                            // <!-- Evento -->
                            <div class="w-full">
                                <label for="event" class="block mb-2 font-medium text-white">
                                    {"¿Cuál es tu interés?"}
                                    *
                                </label>

                                <select
                                    id="event"
                                    name="event"
                                    multiple
                                    required
                                    class="py-3 px-4 w-full placeholder-gray-400 text-gray-900 bg-white rounded-md border border-gray-300 transition appearance-none focus:border-red-500 focus:ring-2 focus:ring-red-500 focus:outline-none"
                                >
                                    <option value="Cursos de Fotografía Dental">
                                        {"Cursos de Fotografía Dental"}
                                    </option>
                                    <option value="Cursos de Rehabilitación Adhesiva y Preparación Dental">
                                        {"Cursos de Rehabilitación Adhesiva y Preparación Dental"}
                                    </option>
                                    <option value="Solicitud de Reserva">
                                        {"Solicitud de Reserva"}
                                    </option>
                                    <option value="Productos de Fotografía Dental">
                                        {"Productos de Fotografía Dental"}
                                    </option>
                                    <option value="Instrumentos para Toma de Color">
                                        {"Instrumentos para Toma de Color"}
                                    </option>
                                    <option value="Fresas Dentales">{"Fresas Dentales"}</option>
                                    <option value="Consulta General">{"Consulta General"}</option>
                                    <option value="Otro">{"Otro"}</option>
                                </select>
                            </div>

                            // <!-- Checkbox -->
                            <label class="flex items-start space-x-2 text-xs text-gray-500">
                                <input
                                    type="checkbox"
                                    required
                                    class="mt-1 w-4 h-4 focus:ring-2 focus:ring-red-500 accent-red-600"
                                />
                                <span>
                                    Al enviar tu información, aceptas recibir correos electrónicos y mensajes de texto con información de marketing de nuestra parte. Puedes darte de baja en cualquier momento.
                                </span>
                            </label>

                            // <!-- Submit -->
                            <button
                                type="submit"
                                class="py-3 w-full font-semibold text-white bg-red-600 rounded-full shadow-md transition transform hover:bg-red-700 hover:shadow-lg active:scale-95"
                            >
                                {"Enviar solicitud"}
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}
