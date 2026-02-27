-- Drop tables in reverse order of dependencies
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS handlers;
DROP TABLE IF EXISTS systems;
-- Drop custom types
DROP TYPE IF EXISTS handler_role;