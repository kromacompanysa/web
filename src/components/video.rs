use leptos::prelude::*;

#[component]
pub fn VideoPlayer(
    src: impl Into<String>,
    autoplay: bool,
    controls: bool,
    loop_: bool,
    muted: bool,
) -> impl IntoView {
    let src = src.into();
    view! {
        <div class="rounded-lg overflow-hidden shadow-lg">
            <video
                class="w-full h-auto"
                src={src}
                autoplay={autoplay}
                controls={controls}
                loop={loop_}
                playsinline
                muted={muted}
            >
                {"Your browser does not support the video tag."}
            </video>
        </div>
    }
}
