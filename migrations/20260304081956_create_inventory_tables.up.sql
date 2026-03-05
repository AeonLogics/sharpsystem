-- Add migration script here
-- Create custom enum types for status tracking
CREATE TYPE tracked_unit_status AS ENUM ('InStock', 'Sold', 'RMA', 'Reserved', 'Archived');
-- 1. Products Table (The Catalog)
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id UUID NOT NULL REFERENCES systems(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    sku VARCHAR(100) UNIQUE,
    category VARCHAR(100),
    is_tracked BOOLEAN NOT NULL DEFAULT FALSE,
    added_by UUID NOT NULL REFERENCES handlers(id),
    last_edited_by UUID REFERENCES handlers(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Index for quick lookups by workspace and SKU
CREATE INDEX idx_products_system_id ON products(system_id);
CREATE INDEX idx_products_sku ON products(sku);
-- 2. Tracked Units Table (Serialized Inventory)
CREATE TABLE tracked_units (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    serial_numbers TEXT [] NOT NULL,
    -- Array to hold 1 to N IMEIs/Serials
    supplier_id UUID,
    -- Nullable for future supplier table
    status tracked_unit_status NOT NULL DEFAULT 'InStock',
    acquisition_cost NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    target_msrp NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    date_added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by UUID NOT NULL REFERENCES handlers(id),
    last_updated_by UUID REFERENCES handlers(id)
);
-- Index for scanning specific IMEIs across the entire inventory extremely fast
CREATE INDEX idx_tracked_units_serial_numbers ON tracked_units USING GIN (serial_numbers);
CREATE INDEX idx_tracked_units_product_id ON tracked_units(product_id);
CREATE INDEX idx_tracked_units_status ON tracked_units(status);
-- 3. Untracked Inventory Table (Bulk Inventory)
CREATE TABLE untracked_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID UNIQUE NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 0 CHECK (quantity >= 0),
    average_acquisition_cost NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    base_retail_price NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    last_updated_by UUID REFERENCES handlers(id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Function to automatically update timestamp columns
CREATE OR REPLACE FUNCTION update_updated_at_column() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ language 'plpgsql';
-- Attach timestamp triggers
CREATE TRIGGER update_products_modtime BEFORE
UPDATE ON products FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_untracked_inventory_modtime BEFORE
UPDATE ON untracked_inventory FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();