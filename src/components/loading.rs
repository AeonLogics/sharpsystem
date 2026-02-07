use leptos::prelude::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="loading-overlay">
            <svg style="display: none;">
                <defs>
                    <filter id="liquid">
                        <feGaussianBlur in="SourceGraphic" stdDeviation="10" result="blur" />
                        <feColorMatrix in="blur" mode="matrix" values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 18 -7" result="liquid" />
                        <feBlend in="SourceGraphic" in2="liquid" />
                    </filter>
                </defs>
            </svg>
            <div class="loading-container">
                <div class="liquid-blob"></div>
                <div class="liquid-blob"></div>
                <div class="liquid-blob"></div>
            </div>
        </div>
    }
}
