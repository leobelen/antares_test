
-- CreateTable
CREATE TABLE "core_log" (
    "id" bigint NOT NULL,
    "log_content" text not null,
    "post_date" timestamptz(6) DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);
