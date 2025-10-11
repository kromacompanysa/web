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
        <video
            class="video-player"
            src={src}
            autoplay={autoplay}
            controls={controls}
            loop={loop_}
            playsinline
            muted={muted}
        >
            {"Your browser does not support the video tag."}
        </video>
    }
}
