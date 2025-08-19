-- Your SQL goes here
-- CreateTable
CREATE TABLE "zap_run_outbox" (
    "id" TEXT NOT NULL,
    "zap_run_id" TEXT NOT NULL,

    CONSTRAINT "zap_run_outbox_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "zap_run_outbox_zap_run_id_key" ON "zap_run_outbox"("zap_run_id");

-- AddForeignKey
ALTER TABLE "zap_run_outbox" ADD CONSTRAINT "zap_run_outbox_zap_run_id_fkey" FOREIGN KEY ("zap_run_id") REFERENCES "zap_run"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AlterTable
ALTER TABLE "zap_run" ADD COLUMN     "metadata" JSONB NOT NULL;