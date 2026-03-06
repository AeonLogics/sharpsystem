# Sharp System Architecture

The Sharp System is designed as a high-performance, multi-tenant "Apex" application built on Rust and Leptos. It is engineered for speed, scale, and professional-grade inventory management.

## Workspace Structure

The project uses a Cargo Workspace to separate concerns across three main crates:

| Crate | Purpose | Key Responsibilities |
| :--- | :--- | :--- |
| **`sharp-system`** | Frontend & Routing | Leptos components, SSR entry point, Hydration logic, and global styles. |
| **`actions`** | Server Logic | Server-side actions (SSR), database interaction (sqlx), and business logic. |
| **`models`** | Shared Core | Shared data structures (Payloads, Entities), Errors, and Traits. |

## Multi-Tenancy (The Apex Layer)

The system supports multiple isolated work environments (Systems). 

- **Systems**: Every business runs on its own `System`. This is the global container for all data.
- **Handlers**: Users linked to a `System` with specific roles (`system_admin`, `system_manager`, `system_salesman`).
- **UI Context**: The `SystemState` (powered by Leptos `RwSignal`) manages the current active workspace and user session.

---

## 📦 Inventory Logic (The Brain)

Sharp System uses a "tri-table" approach to balance high-value tracking with high-volume speed.

### 1. The Catalog (`products` table)
This is the **registry**. It defines *what* a product is (name, category, SKU).
- **The switch**: `is_tracked` (Boolean). This determines how the system handles physical stock.

### 2. Tracked Units (Serialized 📱)
- **Target**: High-value items (e.g., Electronic devices).
- **Storage**: `tracked_units` table.
- **Identity**: Each unit has its own array of unique identifiers (Serials/IMEIs).
- **Logic**: Individual life-cycle tracking (InStock -> Sold -> RMA).

### 3. Untracked Inventory (Bulk 🔌)
- **Target**: High-volume commodities (e.g., Accessories, cables).
- **Storage**: `untracked_inventory` table.
- **Identity**: Tracked as a simple `quantity` integer for maximum performance.
- **Logic**: Fast checkout and bulk stock management.

---

## Data Flow Pattern
1. **Definitions**: Payloads are defined in `models/src/payloads/`.
2. **Validation**: Payloads implement `validate()` to ensure integrity at the edge.
3. **Execution**: Server actions in `actions/` process logic and interact with `db_ops`.
