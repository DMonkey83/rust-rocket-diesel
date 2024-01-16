-- Your SQL goes here
CREATE TABLE "roles" (
  "id" BIGSERIAL PRIMARY KEY NOT NULL,
  "code" VARCHAR(64) NOT NULL UNIQUE,
  "name" VARCHAR(128) NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE "user_roles" (
  "id" BIGSERIAL PRIMARY KEY NOT NULL,
  "user_username" VARCHAR(255) NOT NULL,
  "role_id" BIGINT NOT NULL
);
