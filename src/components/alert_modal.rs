use leptos::prelude::*;
use models::system_state::SystemState;

#[component]
pub fn AlertModal() -> impl IntoView {
    let state = use_context::<RwSignal<SystemState>>().expect("SystemState context not found");

    let close_modal = move |_| {
        state.update(|s| s.clear_modal());
    };

    view! {
        {move || {
            let state = state.get();
            state.modal.as_ref().map(|n| {
                let level_class = format!("{:?}", n.level()).to_lowercase();
                view! {
                    <div class="modal-overlay">
                        <div class={format!("modal-card {}", level_class)}>
                            <div class="modal-header">
                                <div class="modal-icon"></div>
                                <h2 class="modal-title">{n.title()}</h2>
                            </div>
                            <div class="modal-body">
                                <p class="modal-message">{n.message()}</p>
                            </div>
                            <div class="modal-footer">
                                <button on:click=close_modal class="btn btn-primary modal-action">
                                    "Acknowledge"
                                </button>
                            </div>
                        </div>
                    </div>
                }
            })
        }}
    }
}
