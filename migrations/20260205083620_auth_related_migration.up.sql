-- Enable UUID extension (kept if we still need other UUIDs, but primary IDs will be TEXT)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Define custom enum type for roles
CREATE TYPE user_role AS ENUM ('owner', 'admin', 'employee');

-- 1. Companies (Tenants)
CREATE TABLE companies (
    id TEXT PRIMARY KEY, -- ULID with prefix comp_
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Users (Members)
CREATE TABLE users (
    id TEXT PRIMARY KEY, -- ULID with prefix usr_
    company_id TEXT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    role user_role NOT NULL DEFAULT 'employee',
    full_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Sessions
CREATE TABLE sessions (
    id TEXT PRIMARY KEY, -- ULID with prefix ses_
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
