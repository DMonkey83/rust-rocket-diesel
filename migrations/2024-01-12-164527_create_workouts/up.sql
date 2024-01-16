CREATE TABLE WorkoutPlan (
  "id" BIGSERIAL PRIMARY KEY,
  "username" VARCHAR(255) NOT NULL,
  "plan_name" VARCHAR(255) NOT NULL,
  "description" TEXT NOT NULL DEFAULT (''),
  "start_date" timestamptz NOT NULL DEFAULT (now()),
  "end_date" timestamptz NOT NULL DEFAULT (now()),
  "goal" WorkoutGoalEnum NOT NULL DEFAULT ('lose_weight'),
  "difficulty" DifficultyEnum NOT NULL DEFAULT ('beginner'),
  "is_public" VisibilityEnum NOT NULL DEFAULT ('public'),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE Workout (
  "id" BIGSERIAL PRIMARY KEY,
  "workout_name" VARCHAR(255) NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);


CREATE TABLE PlanWorkout (
  "id" BIGSERIAL PRIMARY KEY,
  "plan_id" BIGINT NOT NULL,
  "workout_id" BIGINT NOT NULL,
  "workout_day" WorkoutDayEnum NOT NULL DEFAULT ('monday'),
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE MuscleGroups (
  "id" BIGSERIAL PRIMARY KEY,
  "name" VARCHAR(255) NOT NULL,
  "workout_id" BIGINT NOT NULL,
  "muscle_group" MuscleGroupEnum NOT NULL DEFAULT 'chest'
);
