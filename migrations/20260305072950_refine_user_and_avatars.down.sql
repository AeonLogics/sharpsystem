-- Down migration: Restore user_name and buggy avatar default
ALTER TABLE handlers
ADD COLUMN user_name TEXT;
ALTER TABLE sessions
ADD COLUMN user_name TEXT;
ALTER TABLE systems
ALTER COLUMN avatar_url
SET DEFAULT 'https://api.dicebear.com/7.x/initials/svg?';