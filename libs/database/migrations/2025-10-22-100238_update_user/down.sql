-- This file should undo anything in `up.sql`
ALTER TABLE "user" RENAME COLUMN "name" TO "username";

ALTER TABLE users ADD CONSTRAINT users_username_key UNIQUE (username);

