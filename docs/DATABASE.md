# Sharp System: Database Architecture ◈ 🗄️

Sharp System uses a high-performance relational schema powered by PostgreSQL and SQLx. The design prioritizes strict data isolation and efficient inventory tracking.

## ◈ Core Schema Design

### 1. Multi-Tenant Isolation
Every table is fundamentally linked to a `system_id` (UUID).
- **Security**: All queries must include the tenant check to ensure zero data leakage between Systems.
- **Indexes**: Every table has a `B-TREE` index on `system_id` for $O(\log n)$ lookup speeds.

### 2. The Product Logic (Tri-Table)
The inventory is split into three layers to balance grain and speed:
- **`products`**: The catalog definitions. Contains the `is_tracked` toggle.
- **`tracked_units`**: Serialized inventory. Uses `GIN` indexes on the `serial_numbers` (TEXT[]) column for lightning-fast IMEI/Serial scans.
- **`untracked_inventory`**: Bulk inventory. A simple counter for high-volume accessories.

---

## ◈ Key Indexes & Performance
| Index Type | Targeted Operation |
| :--- | :--- |
| **B-TREE** | System IDs, SKUs (Unique), and Foreign Keys. |
| **GIN** | Serial Number searching in the `tracked_units` array. |
| **TIMESTAMP** | Used on `updated_at` for efficient delta-syncing. |

## ◈ Migration Strategy
- **Version Control**: Managed via `sqlx-migrations`.
- **Naming**: `YYYYMMDDHHMMSS_description.up.sql`.
- **Constraints**: Use `ON DELETE CASCADE` appropriately to maintain referential integrity in a multi-tenant environment.
