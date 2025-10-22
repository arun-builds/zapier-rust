-- Your SQL goes here
ALTER TABLE users DROP CONSTRAINT users_username_key;

ALTER TABLE "users" RENAME COLUMN "username" TO "name";

