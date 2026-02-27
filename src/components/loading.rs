use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn Loading(
    #[prop(optional)] size: Option<f64>,
    #[prop(default = true)] full_screen: bool,
) -> impl IntoView {
    let ripples = RwSignal::new(Vec::<(f64, f64, u64)>::new());

    let on_click = move |ev: ev::MouseEvent| {
        if !full_screen {
            return;
        } // Only interactive in full screen for now
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        let id = (web_sys::window().unwrap().performance().unwrap().now() * 1000.0) as u64;

        ripples.update(|r| r.push((x, y, id)));

        set_timeout(
            move || {
                ripples.update(|r| r.retain(|&(_, _, rid)| rid != id));
            },
            std::time::Duration::from_millis(1000),
        );
    };

    let svg_style = size
        .map(|s| format!("width: {}px; height: {}px;", s, s * 1.125))
        .unwrap_or_else(|| "width: 160px; height: 180px;".to_string());

    let overlay_class = if full_screen {
        "loading-overlay interactive"
    } else {
        "loading-embedded"
    };

    view! {
        <div class=overlay_class on:mousedown=on_click>
            {move || if full_screen {
                view! {
                    <div class="ripple-container">
                        <For
                            each=move || ripples.get()
                            key=|(_, _, id)| *id
                            let:ripple
                        >
                            {
                                let (x, y, _) = ripple;
                                let style = format!("left: {}px; top: {}px;", x, y);
                                view! { <div class="ripple-pulse" style=style></div> }
                            }
                        </For>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}

            <div class="chronos-weaver bolder">
                <svg viewBox="0 0 100 120" style=svg_style class="hourglass-svg">
                    // The Structural Braces
                    <rect x="15" y="15" width="70" height="90" rx="4" class="frame-brace" />

                    // The Primary Frame - Extra Bold
                    <path d="M20 20 L80 20 L50 60 L80 100 L20 100 L50 60 Z" class="frame-path" />

                    // Span-Lines: Paths that take more than one span
                    <path d="M10 60 L90 60 L50 10 L50 110 Z" class="frame-span" />

                    // The "Sand" Flow
                    <g class="sand-particles">
                        {(0..10).map(|i| {
                            let delay = format!("-{}s", i as f32 * 0.25);
                            view! { <circle cx="50" cy="20" r="2.5" style=format!("animation-delay: {}", delay) /> }
                        }).collect_view()}
                    </g>
                </svg>
            </div>
        </div>
    }
}

#[component]
pub fn InlineLoader(
    #[prop(optional, into)] loading_text: Option<String>,
    #[prop(default = "size-base")] size_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="inline-loader-container flex items-center gap-3">
            <div class=format!("music-bars {}", size_class)>
                <span class="bar bar-1"></span>
                <span class="bar bar-2"></span>
                <span class="bar bar-3"></span>
                <span class="bar bar-4"></span>
                <span class="bar bar-5"></span>
            </div>
            {loading_text.map(|text| view! {
                <span class="loading-text text-sm font-mono text-muted animate-pulse">{text}</span>
            })}
        </div>
    }
}
