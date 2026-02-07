use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn ThemedInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] set_value: WriteSignal<String>,
    #[prop(into)] label: String,
    #[prop(optional, into)] input_type: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let input_type = if input_type.is_empty() {
        "text".to_string()
    } else {
        input_type
    };
    let id_val = if id.is_empty() {
        format!("input-{}", NEXT_ID.fetch_add(1, Ordering::SeqCst))
    } else {
        id
    };

    view! {
        <div class=move || {
            format!(
                "input-group {}",
                class,
            )
        }>
            <input
                type=input_type
                class="input-field"
                placeholder=" "
                prop:value=value
                on:input=move |ev| set_value.set(event_target_value(&ev))
                id={id_val.clone()}
            />
            <label class="input-label" for={id_val}>
                {label}
            </label>
        </div>
    }
}
