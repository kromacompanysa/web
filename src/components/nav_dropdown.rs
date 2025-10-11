use leptos::prelude::*;

#[component]
pub fn HeaderDropdownMenu(
    title: &'static str,
    items: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    view! {
        <div class="relative group">
            <button class="flex items-center space-x-1 transition hover:text-blue-600">
                <span>{title}</span>
                <svg
                    class="w-4 h-4 transition transform group-hover:rotate-180"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                >
                    <path
                        fill-rule="evenodd"
                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.2l3.71-3.97a.75.75 0 111.08 1.04l-4.25 4.55a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z"
                        clip-rule="evenodd"
                    />
                </svg>
            </button>
            <div class="absolute left-0 invisible w-64 bg-white rounded-lg shadow-lg opacity-0 transition duration-200 group-hover:visible group-hover:opacity-100">
                <ul class="py-2">
                    <For
                        each={move || items.iter().cloned()}
                        key={|(href, _label)| href.to_string()}
                        children={move |(href, label)| {
                            view! {
                                <li>
                                    <a href={href} class="block py-2 px-4 hover:bg-gray-100">
                                        {label}
                                    </a>
                                </li>
                            }
                        }}
                    />
                </ul>
            </div>
        </div>
    }
}
