use super::components::AddProductForm;
use leptos::prelude::*;

#[component]
pub fn CatalogPage() -> impl IntoView {
    let is_adding = RwSignal::new(false);
    let product_id = RwSignal::new(None);
    let products = Resource::new(move || product_id.get(), |id: Option<String>| async move {});
    view! {
        <div class="system-page-container">
            <header class="system-page-header">
                <div>
                    <h1>"Product Catalog"</h1>
                    <p>"Manage your menu of products and decide what items to track."</p>
                </div>
                <button
                    class="btn btn-primary shadow-sm hover:-translate-y-1 transition-all"
                    on:click=move |_| is_adding.set(!is_adding.get())
                >
                    {move || if is_adding.get() { "Cancel" } else { "+ Add Product" }}
                </button>
            </header>

            // AI Info Box explaining Tracked vs Untracked
            <div class="info-box shadow-sm">
                <div class="info-icon">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10"></circle>
                        <path d="M12 16v-4"></path>
                        <path d="M12 8h.01"></path>
                    </svg>
                </div>
                <div>
                    <h4>"AI Asssistant: What should I track?"</h4>
                    <p>
                        "When you add a product to the catalog, you must decide if it is "
                        <strong>"Tracked"</strong> " or " <strong>"Untracked"</strong> "." <br />
                        "• " <strong>"Tracked Items"</strong>
                        " (like iPhones) force you to scan a unique IMEI or Serial Number for every single unit you sell. This protects high-value inventory."
                        <br /> "• " <strong>"Untracked Items"</strong>
                        " (like cables) are treated as bulk quantities. You just say 'We have 50 of these' without scanning individual serials."
                    </p>
                </div>
            </div>

            <Show when=move || is_adding.get()>
                <AddProductForm />
            </Show>

            // Future Data Table
            <div class="system-content-area">
                <div class="empty-state">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="48"
                        height="48"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                        <polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
                        <line x1="12" y1="22.08" x2="12" y2="12"></line>
                    </svg>
                    <h3>"No Products Found"</h3>
                    <p>
                        "Your catalog is empty. Click '+ Add Product' above to define your first menu item before adding physical stock."
                    </p>
                </div>
            </div>
        </div>
    }
}
