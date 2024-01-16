CREATE TYPE GenderEnum AS ENUM (
  'male',
  'female'
);

CREATE TYPE WeightEnum AS ENUM (
  'kg',
  'lb'
);

CREATE TYPE HeightEnum AS ENUM (
  'cm',
  'ft_in'
);
-- Your SQL goes here
CREATE TABLE "users" (
  "username" VARCHAR(255) PRIMARY KEY NOT NULL,
  "password_hash" VARCHAR(255) NOT NULL,
  "password_changed_at" timestamptz NOT NULL DEFAULT '0001-01-01 00:00:00z',
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE UserProfile (
  "username" VARCHAR(255) UNIQUE PRIMARY KEY NOT NULL,
  "first_name" VARCHAR(255) NOT NULL,
  "last_name" VARCHAR(255) NOT NULL,
  "email" VARCHAR(255) NOT NULL,
  "age" INT NOT NULL,
  "gender" GenderEnum NOT NULL,
  "height" INT NOT NULL,
  "preferred_weight_unit" WeightEnum NOT NULL,
  "preferred_height_unit" HeightEnum NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE UNIQUE INDEX "unique_email" ON UserProfile ("email");

ALTER TABLE UserProfile ADD FOREIGN KEY ("username") REFERENCES "users" ("username");
