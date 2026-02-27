use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <main class="not-found-page">
            <div class="gradient-orb orb-4"></div>
            <div class="gradient-orb orb-5"></div>

            <section class="not-found-container center">
                <div class="glass-card not-found-card slide-up">
                    <div class="not-found-visual">
                        <span class="error-code">"404"</span>
                        <div class="glitch-effect" data-text="404">"404"</div>
                    </div>

                    <h1 class="not-found-title">"Looks like you're lost hahah"</h1>
                    <p class="not-found-subtitle">
                        "The page you are looking for has drifted into the void. <br/>
                        Don't worry, even the best systems need a reboot sometimes."
                    </p>

                    <div class="not-found-actions">
                        <A href="/" attr:class="btn btn-primary btn-lg glow">"Return to Base"</A>
                        <button on:click=|_| {
                            let window = web_sys::window().expect("no global `window` exists");
                            let history = window.history().expect("should have history");
                            let _ = history.back();
                        } class="btn btn-ghost">"Go Back"</button>
                    </div>
                </div>
            </section>
        </main>
    }
}
