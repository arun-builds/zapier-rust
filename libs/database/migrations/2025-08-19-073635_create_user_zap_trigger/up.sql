-- CreateTable
CREATE TABLE "user" (
    "id" SERIAL NOT NULL,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,

    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "zap" (
    "id" TEXT NOT NULL,
    "trigger_id" TEXT NOT NULL,

    CONSTRAINT "zap_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "trigger" (
    "id" TEXT NOT NULL,
    "zap_id" TEXT NOT NULL,
    "trigger_id" TEXT NOT NULL,

    CONSTRAINT "trigger_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "action" (
    "id" TEXT NOT NULL,
    "zap_id" TEXT NOT NULL,
    "action_id" TEXT NOT NULL,

    CONSTRAINT "action_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "available_action" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "available_action_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "available_trigger" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "available_trigger_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "trigger_zap_id_key" ON "trigger"("zap_id");

-- AddForeignKey
ALTER TABLE "trigger" ADD CONSTRAINT "trigger_trigger_id_fkey" FOREIGN KEY ("trigger_id") REFERENCES "available_trigger"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "trigger" ADD CONSTRAINT "trigger_zap_id_fkey" FOREIGN KEY ("zap_id") REFERENCES "zap"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "action" ADD CONSTRAINT "action_zap_id_fkey" FOREIGN KEY ("zap_id") REFERENCES "zap"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "action" ADD CONSTRAINT "action_action_id_fkey" FOREIGN KEY ("action_id") REFERENCES "available_action"("id") ON DELETE RESTRICT ON UPDATE CASCADE;