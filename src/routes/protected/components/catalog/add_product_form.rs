use actions::AddProduct;
use leptos::prelude::*;
use models::shared::notifications::{Notification, NotificationLevel};
use models::system_state::SystemState;
use std::sync::Arc;

#[component]
pub fn AddProductForm() -> impl IntoView {
    let state =
        use_context::<RwSignal<SystemState>>().expect("SystemState context should be provided");
    let add_product_action = ServerAction::<AddProduct>::new();
    let is_adding = RwSignal::new(false);
    let submit_value = add_product_action.value();

    // Watch for server errors and display them as toasts
    Effect::new(move |_| {
        if let Some(Err(e)) = submit_value.get() {
            state.update(|s| {
                s.add_toast(Arc::new(e));
            });
        }
    });

    // Watch for success, close modal, and display success toast
    Effect::new(move |_| {
        if let Some(Ok(product)) = submit_value.get() {
            is_adding.set(false);
            state.update(|s| {
                s.add_toast(Arc::new(Notification::new(
                    "Product Added",
                    &format!("Successfully added '{}' to the catalog.", product.name),
                    NotificationLevel::Success,
                )));
            });
        }
    });

    view! {
        <ActionForm action=add_product_action>
            <div class="catalog-add-form">
                <div class="form-header">
                    <h3>"Define New Product"</h3>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    // Product Name Input
                    <div class="input-group">
                        <input
                            type="text"
                            name="name"
                            class="input-field"
                            placeholder=" "
                            required
                        />
                        <label class="input-label">"Product Name (e.g. iPhone 15 Pro)"</label>
                    </div>

                    // Category Input
                    <div class="input-group">
                        <input type="text" name="category" class="input-field" placeholder=" " />
                        <label class="input-label">"Category (e.g. Smartphones)"</label>
                    </div>

                    // SKU Input
                    <div class="input-group">
                        <input type="text" name="sku" class="input-field" placeholder=" " />
                        <label class="input-label">"Internal SKU (Optional)"</label>
                    </div>
                </div>

                // THE MASSIVE TOGGLE SWITCH
                <label class="tracking-toggle mt-2">
                    <div class="toggle-info">
                        <strong>"Track Individual Units?"</strong>
                        <span>
                            "Turn this ON for serial-numbered items (phones, tablets). Leave OFF for bulk items (cables, cases)."
                        </span>
                    </div>
                    <div class="switch">
                        <input type="checkbox" name="is_tracked" value="true" />
                        <span class="slider"></span>
                    </div>
                </label>

                <div class="flex justify-end mt-4 pt-4 border-t border-[var(--border-default)]">
                    <input
                        type="submit"
                        class="btn btn-primary"
                        value="Save Product to Catalog"
                        disabled=move || add_product_action.pending().get()
                    />
                </div>
            </div>
        </ActionForm>
    }
}
