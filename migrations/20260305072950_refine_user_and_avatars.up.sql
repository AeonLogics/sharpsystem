-- Up migration: Refine user and avatar settings
ALTER TABLE handlers DROP COLUMN IF EXISTS user_name;
ALTER TABLE sessions DROP COLUMN IF EXISTS user_name;
ALTER TABLE systems
ALTER COLUMN avatar_url
SET DEFAULT 'https://api.dicebear.com/7.x/initials/svg';