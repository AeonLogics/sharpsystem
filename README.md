# SharpSystem V2 ◈ 📊

The high-performance migration of **SharpSystem**—a multi-tenant financial operations platform built for speed, safety, and scale. Moving from Next.js to a purely native Rust stack with **Leptos**, **Axum**, and **SQLx**.

## ◈ Project Vision

SharpSystem is engineered to handle high-precision data operations with zero-cost abstractions.
- **Financial Integrity**: Native Rust decimal handling for Bills and Receipts (no floating point errors).
- **The "Sharp" Pipeline**: A dedicated native image processing engine for high-speed document scanning.
- **Micro-Latency**: Sub-millisecond routing and hydration via WebAssembly and Axum.
- **Multi-Tenant First**: Secure, organization-locked data silos using Prefixed ULIDs.

## ◈ System Documentation

Detailed implementation patterns can be found in the `docs/` directory:

- [**System Architecture**](file:///d:/RustRover/sharp-system/docs/architecture.md): Multi-crate workspace and Bento architecture patterns.
- [**Authentication Flow**](file:///d:/RustRover/sharp-system/docs/auth_flow.md): Secure bootstrap and session management.
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
```bash
# Run in development mode with hot-reload
cargo leptos watch
```

### 3. Production Build
```bash
cargo leptos build --release
```

## ◈ License
SharpSystem Proprietary Implementation.
