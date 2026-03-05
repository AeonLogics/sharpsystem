-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE handler_role AS ENUM (
    'system_admin',
    'system_manager',
    'system_salesman'
);
-- 1. Apex System (Global Authority)
CREATE TABLE systems(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID,
    system_handle TEXT UNIQUE,
    system_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    avatar_url TEXT DEFAULT 'https://api.dicebear.com/7.x/initials/svg?'
);
-- 2. Node Handlers (Branded Identity)
CREATE TABLE handlers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id UUID REFERENCES systems(id) ON DELETE CASCADE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    user_name TEXT NOT NULL,
    handler_role handler_role DEFAULT 'system_salesman',
    avatar_url TEXT,
    bio TEXT,
    preferred_theme TEXT DEFAULT 'light',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- 3. Unified Session Store
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id UUID REFERENCES systems(id) ON DELETE CASCADE,
    handler_id UUID REFERENCES handlers(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    handler_role handler_role NOT NULL,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL,
    avatar_url TEXT,
    bio TEXT,
    preferred_theme TEXT,
    system_handle TEXT NOT NULL,
    system_name TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '7 days'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT session_identity_check CHECK (
        (
            system_id IS NOT NULL
            AND handler_id IS NOT NULL
        )
    )
);