use leptos::prelude::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <SimpleCounter initial_value=0 step=1 /> }
    })
}

#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = signal(initial_value);

    view! {
        <div class="flex gap-4 items-center p-4 bg-gray-100 rounded-lg">
            <button
                class="py-2 px-4 text-white bg-red-500 rounded transition-colors hover:bg-red-600"
                on:click={move |_| set_value.set(0)}
            >
                "Clear"
            </button>
            <button
                class="py-2 px-4 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
                on:click={move |_| *set_value.write() -= step}
            >
                "-1"
            </button>
            <span class="text-xl font-semibold text-center min-w-[120px]">
                "Value: " {value} "!"
            </span>
            <button
                class="py-2 px-4 text-white bg-green-500 rounded transition-colors hover:bg-green-600"
                on:click={move |_| set_value.update(|value| *value += step)}
            >
                "+1"
            </button>
        </div>
    }
}
