-- Add migration script here
-- -- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- If you have an existing table, drop the INT id
ALTER TABLE users DROP COLUMN id;

-- Add UUID id
ALTER TABLE users
ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();
