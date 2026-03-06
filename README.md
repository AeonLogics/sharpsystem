# SharpSystem V2 ◈ 📊

The high-performance migration of **SharpSystem**—a multi-tenant financial operations platform built for speed, safety, and scale. Moving from Next.js to a purely native Rust stack with **Leptos**, **Axum**, and **SQLx**.

SharpSystem is engineered to handle high-precision data operations with zero-cost abstractions.
- **Financial & Inventory Integrity**: Native Rust handling for stock levels and financial records (no float errors).
- **The "Sharp" Architecture**: High-performance "Apex" architecture with multi-tenant isolation.
- **Micro-Latency**: Sub-millisecond routing and hydration via WebAssembly and Axum.
- **Apex Isolation**: Secure, System-level data silos using UUID-v4 with strict foreign key constraints.

## ◈ System Documentation

Detailed implementation patterns can be found in the `docs/` directory:

- [**System Architecture**](file:///d:/RustRover/sharp-system/docs/architecture.md): Multi-crate workspace and Apex isolation logic.
- [**Inventory & Database**](file:///d:/RustRover/sharp-system/docs/DATABASE.md): Schema design and Tracked vs. Untracked logic.
- [**Authentication Flow**](file:///d:/RustRover/sharp-system/docs/auth_flow.md): Secure bootstrap and session management.
- [**API & Server Actions**](file:///d:/RustRover/sharp-system/docs/API_PATTERNS.md): Type-safe frontend-to-backend protocols.
- [**UI/UX Guide**](file:///d:/RustRover/sharp-system/docs/UI_UX_GUIDE.md): The "Sharp Terminal" aesthetic and animation principles.
- [**Notification Engine**](file:///d:/RustRover/sharp-system/docs/notifications.md): Premium stackable toaster and modal documentation.

## ◈ Tech Stack

- **Frontend**: [Leptos](https://leptos.dev/) (Full-stack Rust framework)
- **Runtime**: [Axum](https://github.com/tokio-rs/axum) (High-concurrency async web server)
- **Database**: [Postgres](https://www.postgresql.org/) + [SQLx](https://github.com/launchbadge/sqlx) (Compile-time verified SQL)
- **Styling**: Modular SCSS with "Sharp Terminal" Aesthetic.

## ◈ Setup & Initialization

### 1. Prerequisites
- Rust Nightly (`rustup toolchain install nightly`)
- `cargo-leptos` (`cargo install cargo-leptos`)
- `dart-sass` (`npm install -g sass`)

### 2. Startup
```powershell
# Initialize environment
./setup.ps1

# Run in development mode with hot-reload
cargo leptos watch
```

### 3. Production Build
```bash
cargo leptos build --release
```

## ◈ License

**Proprietary & Confidential.** All Rights Reserved (c) 2026.
Unauthorized use or distribution is strictly prohibited.
