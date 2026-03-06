# Sharp System: API & Server Action Patterns ◈ 🔌

Sharp System utilizes the Leptos "Server Action" pattern to ensure type-safe, validated communication between the WASM frontend and the Rust backend.

## ◈ The Payload Pattern

To maintain high efficiency and clean code, every server interaction follows a 4-step sequence:

### 1. Model Definition (`models/src/payloads/`)
Payloads are defined as `derive(Deserialize, Serialize, Validate)` structs.
```rust
pub struct AddProductPayload {
    pub name: String,
    pub sku: Option<String>,
    // ...
}
```

### 2. Validation (`models/src/payloads/`)
Each payload implements logic to catch errors at the edge before hitting the database.

### 3. Server Action (`actions/src/`)
Server functions are annotated with `#[server]`. They handle session extraction and DB pool acquisition.
```rust
#[server(AddProduct)]
pub async fn add_product(payload: AddProductPayload) -> Result<Product, SystemError> {
    // Session check -> Payload validate -> DB Op
}
```

### 4. Database Ops (`actions/src/db_ops/`)
Pure database logic is isolated in the `db_ops` module, keeping the `server` functions focused on orchestration.

---

## ◈ Error Handling (`models/src/errors.rs`)
The system uses a unified `SystemError` enum that maps directly to:
- **API Responses**: Descriptive JSON errors for the frontend.
- **Toasts**: Auto-formatted notifications via the Notification Engine.

## ◈ Session Context
- **Extraction**: Handled via `leptos_axum::extract()`.
- **Authority**: The session token is used to resolve the `Handler` and their associated `System_ID`.
