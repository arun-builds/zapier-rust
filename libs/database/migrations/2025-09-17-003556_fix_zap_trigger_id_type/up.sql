-- Your SQL goes here
-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE "zap" ALTER COLUMN trigger_id TYPE UUID USING trigger_id::uuid;