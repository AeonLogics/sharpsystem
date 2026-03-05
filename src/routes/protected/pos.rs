use leptos::prelude::*;

#[component]
pub fn PosPage() -> impl IntoView {
    view! {
        <div class="pos-layout">
            // Left Pane: The Active Invoice
            <div class="pos-cart-pane">
                <div class="cart-header">
                    <h2 class="text-lg font-bold text-primary">"Current Sale"</h2>
                    <p class="text-secondary text-sm">"Invoice #REQ-001"</p>
                </div>

                <div class="cart-items empty-state">
                    <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="21" r="1"></circle><circle cx="20" cy="21" r="1"></circle><path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path></svg>
                    <h3>"Cart is Empty"</h3>
                    <p>"Scan an item to add it to the cart."</p>
                </div>

                <div class="cart-footer">
                    <div class="totals-row">
                        <span class="text-secondary">"Subtotal"</span>
                        <span class="text-primary">"$0.00"</span>
                    </div>
                    <div class="totals-row grand-total">
                        <span class="text-primary">"Total"</span>
                        <span class="amount">"$0.00"</span>
                    </div>
                    <button class="btn btn-primary w-full text-lg font-bold py-4">
                        "Complete Sale"
                    </button>
                </div>
            </div>

            // Right Pane: Universal Search & Catalog Grid
            <div class="pos-scanner-pane">
                // AI Info Box explaining Universal Search
                <div class="info-box shadow-sm">
                    <div class="info-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><path d="M12 16v-4"></path><path d="M12 8h.01"></path></svg>
                    </div>
                    <div>
                        <h4>"AI Assistant: The Universal Scanner"</h4>
                        <p>
                            "You don't need to click around! Just focus on the search bar below."<br/>
                            "• " <strong>"Scan an IMEI"</strong> " -> We instantly find the specific Tracked phone and add it to the receipt."<br/>
                            "• " <strong>"Type 'Cable'"</strong> " -> We find all matching Untracked items so you can specify a quantity."
                        </p>
                    </div>
                </div>

                <div class="scanner-input-wrapper">
                    <div class="search-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                    </div>
                    <input
                        type="text"
                        placeholder="Scan Barcode, IMEI, or Search Product..."
                        autofocus
                    />
                </div>

                <div class="system-content-area border-dashed">
                     <div class="empty-state">
                         <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="mb-4 opacity-50"><path d="M4 7V4h16v3M9 20h6M12 4v16"/></svg>
                         <h3>"Scanner Active"</h3>
                         <p>"The system is listening. Waiting for input..."</p>
                     </div>
                </div>
            </div>
        </div>
    }
}
