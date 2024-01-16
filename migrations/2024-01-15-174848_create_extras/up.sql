-- Your SQL goes here
CREATE TABLE Set (
  "id" BIGSERIAL PRIMARY KEY,
  "exercise_log_id" BIGINT NOT NULL,
  "set_number" INT NOT NULL DEFAULT (1),
  "weight" int NOT NULL DEFAULT (1),
  "rest_duration" VARCHAR(8) NOT NULL DEFAULT (''),
  "reps_completed" INT NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE WeightEntry (
  "id" BIGSERIAL PRIMARY KEY,
  "username" VARCHAR(255) NOT NULL,
  "entry_date" timestamptz NOT NULL DEFAULT (now()),
  "weight" INT NOT NULL DEFAULT (0),
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE MaxRepGoal (
  "id" BIGSERIAL PRIMARY KEY,
  "username" VARCHAR(255) NOT NULL,
  "exercise_name" VARCHAR(255) UNIQUE NOT NULL,
  "goal_reps" INT NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE MaxWeightGoal (
  "id" BIGSERIAL PRIMARY KEY,
  "username" VARCHAR(255) NOT NULL,
  "exercise_name" VARCHAR(255) UNIQUE NOT NULL,
  "goal_weight" INT NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);
