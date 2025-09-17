CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- Your SQL goes here


CREATE TABLE "available_trigger" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "name" TEXT NOT NULL
);

CREATE TABLE "available_action" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "name" TEXT NOT NULL
);

CREATE TABLE "zap" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "trigger_id" TEXT NOT NULL,
    "user_id" UUID NOT NULL,
    FOREIGN KEY ("user_id") REFERENCES users("id") ON DELETE RESTRICT ON UPDATE CASCADE
    
);

CREATE TABLE "trigger" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "zap_id" UUID NOT NULL UNIQUE,
    "trigger_id" UUID NOT NULL,
    "metadata" JSONB NOT NULL DEFAULT '{}',
    FOREIGN KEY ("zap_id") REFERENCES "zap"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY ("trigger_id") REFERENCES "available_trigger"("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE "action" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "zap_id" UUID NOT NULL,
    "action_id" UUID NOT NULL,
    "metadata" JSONB NOT NULL DEFAULT '{}',
    "sorting_order" INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY ("zap_id") REFERENCES "zap"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    FOREIGN KEY ("action_id") REFERENCES "available_action"("id") ON DELETE RESTRICT ON UPDATE CASCADE

);

CREATE TABLE "zap_run" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "zap_id" UUID NOT NULL,
    "metadata" JSONB NOT NULL,
    FOREIGN KEY ("zap_id") REFERENCES "zap"("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE "zap_run_outbox" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "zap_run_id" UUID NOT NULL,
    FOREIGN KEY ("zap_run_id") REFERENCES "zap_run"("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
    