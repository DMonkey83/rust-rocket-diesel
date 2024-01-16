-- Your SQL goes here
ALTER TABLE MuscleGroups ADD FOREIGN KEY ("workout_id") REFERENCES Workout ("id");

ALTER TABLE WorkoutPlan ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE PlanWorkout ADD FOREIGN KEY ("plan_id") REFERENCES WorkoutPlan ("id");

ALTER TABLE PlanWorkout ADD FOREIGN KEY ("workout_id") REFERENCES Workout ("id");

