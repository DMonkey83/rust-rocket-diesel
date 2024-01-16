-- Your SQL goes here
ALTER TABLE MuscleGroups ADD FOREIGN KEY ("workout_id") REFERENCES Workout ("id");
