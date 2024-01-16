-- Your SQL goes here
CREATE TABLE WorkoutExercises (
  "id" BIGSERIAL PRIMARY KEY,
  "workout_id" BIGINT NOT NULL,
  "exercise_name" VARCHAR(255) NOT NULL,
  "sets" INT NOT NULL DEFAULT (0),
  "reps" VARCHAR(255) NOT NULL DEFAULT (0),
  "rest_duration" VARCHAR NOT NULL DEFAULT ('')
);

CREATE TABLE Exercise (
  "exercise_name" VARCHAR(255) PRIMARY KEY NOT NULL,
  "equipment_required" EquipmentEnum NOT NULL DEFAULT ('barbell'),
  "description" TEXT NOT NULL DEFAULT (''),
  "instructions" TEXT NOT NULL DEFAULT (''),
  "muscle_group_name" MuscleGroupEnum NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE WorkoutLog (
  "id" BIGSERIAL PRIMARY KEY,
  "username" VARCHAR NOT NULL,
  "plan_id" BIGINT NOT NULL,
  "workout_id" BIGINT NOT NULL,
  "log_date" timestamptz NOT NULL DEFAULT (now()),
  "rating" RatingEnum NOT NULL DEFAULT ('one'),
  "fatigue_level" FatigueLevelEnum NOT NULL DEFAULT ('very_light'),
  "overall_feeling" TEXT NOT NULL DEFAULT (''),
  "comments" TEXT NOT NULL DEFAULT (''),
  "workout_duration" VARCHAR(10) NOT NULL DEFAULT ('0m'),
  "total_calories_burned" INT NOT NULL DEFAULT (0),
  "total_distance" INT NOT NULL DEFAULT (0),
  "total_repetitions" INT NOT NULL DEFAULT (0),
  "total_sets" INT NOT NULL DEFAULT (0),
  "total_weight_lifted" INT NOT NULL DEFAULT (0),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE ExerciseLog (
  "id" BIGSERIAL PRIMARY KEY,
  "log_id" BIGINT NOT NULL,
  "exercise_name" VARCHAR(255) NOT NULL,
  "sets_completed" INT NOT NULL DEFAULT (0),
  "repetitions_completed" INT NOT NULL DEFAULT (0),
  "weight_lifted" INT NOT NULL DEFAULT (0),
  "notes" VARCHAR(255) NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);
