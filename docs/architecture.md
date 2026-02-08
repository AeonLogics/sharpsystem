# Sharp System Architecture

The Sharp System is designed as a high-performance, multi-tenant "terminal" application built on Rust and Leptos.

## Workspace Structure

The project uses a Cargo Workspace to separate concerns across three main crates:

| Crate | Purpose | Key Responsibilities |
| :--- | :--- | :--- |
| **`sharp-system`** | Frontend & Routing | Leptos components, SSR entry point, Hydration logic, and global styles. |
| **`actions`** | Server Logic | Server-side actions (SSR), database interaction (sqlx), and business logic. |
| **`models`** | Shared Core | Shared data structures (Payloads, Entities), Errors, and Traits used by both frontend and backend. |

## Data Flow Pattern (Bento Architecture)

We use a "Payload Pattern" to ensure type safety and validation between the frontend UI and backend server actions.

1. **Definitions**: Payloads are defined in `models/src/payloads/`.
2. **Validation**: Each payload implements a `validate()` method to ensure data integrity before reaching the database.
3. **Consumption**: Server actions in `actions/` consume these payloads and interact with the database entities defined in `models/src/entities/`.

## Multi-Tenancy

The system is architected to support multiple companies (tenants). Every user is linked to a `Company` via the `company_id` foreign key.

- **Self-Service Registration**: Single registration flow creates both a new `Company` and its initial `Owner` user.
- **UI Context**: The `SystemState` (powered by Leptos `RwSignal`) manages the current active company and user session.
