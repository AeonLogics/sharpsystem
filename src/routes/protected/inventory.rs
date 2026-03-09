use leptos::prelude::*;

#[component]
pub fn InventoryPage() -> impl IntoView {
    view! {
        <div class="system-page-container">
            <header class="system-page-header">
                <div>
                    <h1>"Inventory Shipments"</h1>
                    <p>"Receive physical stock and attach it to your catalog products."</p>
                </div>
            </header>

            <div class="tabs-header">
                <button class="tab-btn active">"Tracked Units (Rapid Scan)"</button>
                <button class="tab-btn">"Untracked (Bulk Stock)"</button>
            </div>

            // AI Info Box explaining HOW to use the scanner
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
                    <h4>"AI Assistant: How do I receive a shipment?"</h4>
                    <p>
                        "1. First, select the Product (e.g., iPhone 15) from the dropdown above."
                        <br /> "2. Click inside the scanner box below."<br />
                        "3. Start scanning the barcodes on the boxes. Our system will instantly capture all IMEIs and save 50 units in under a second."
                    </p>
                </div>
            </div>

            <div class="system-content-area border-dashed">
                // Future Scanning Engine UI goes here
                <div class="empty-state">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="48"
                        height="48"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="mb-4 opacity-50"
                    >
                        <path d="M3 7V5a2 2 0 0 1 2-2h2"></path>
                        <path d="M17 3h2a2 2 0 0 1 2 2v2"></path>
                        <path d="M21 17v2a2 2 0 0 1-2 2h-2"></path>
                        <path d="M7 21H5a2 2 0 0 1-2-2v-2"></path>
                        <path d="M8 7v10"></path>
                        <path d="M12 7v10"></path>
                        <path d="M16 7v10"></path>
                    </svg>
                    <h3>"Ready to Scan"</h3>
                    <p>"Waiting for barcode input..."</p>
                </div>
            </div>
        </div>
    }
}
