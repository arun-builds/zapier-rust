CREATE TABLE "zap_run" (
    "id" TEXT NOT NULL,
    "zap_id" TEXT NOT NULL,

    CONSTRAINT "zap_run_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "zap_run" ADD CONSTRAINT "zap_run_zap_id_fkey" FOREIGN KEY ("zap_id") REFERENCES "zap"("id") ON DELETE RESTRICT ON UPDATE CASCADE;