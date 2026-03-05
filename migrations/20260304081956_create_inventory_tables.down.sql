-- Revert migration script here
DROP TRIGGER IF EXISTS update_untracked_inventory_modtime ON untracked_inventory;
DROP TRIGGER IF EXISTS update_products_modtime ON products;
DROP FUNCTION IF EXISTS update_updated_at_column();
DROP TABLE IF EXISTS untracked_inventory;
DROP TABLE IF EXISTS tracked_units;
DROP TABLE IF EXISTS products;
DROP TYPE IF EXISTS tracked_unit_status;