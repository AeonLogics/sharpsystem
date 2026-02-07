-- Drop tables in reverse order of dependencies
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS companies;

-- Drop custom types
DROP TYPE IF EXISTS user_role;
