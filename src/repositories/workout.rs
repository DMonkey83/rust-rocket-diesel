use diesel::{ExpressionMethods,QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{workout::{NewWorkout, Workout, WorkoutResponse}, muscle_group::MuscleGroup},
    schema::workout,
};

pub struct WorkoutRepository;

impl WorkoutRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<WorkoutResponse> {
        let workout: Workout = workout::table.find(id).get_result(c).await?;
        let muscle_groups = crate::repositories::muscle_groups::MuscleGroupRepository::list_by_workout_id(c, &workout.id).await?;
        return Ok(WorkoutResponse {
            id: workout.id,
            workout_name: workout.workout_name,
            notes: workout.notes,
            created_at: workout.created_at,
            muscle_groups,
        });
    }

    pub async fn find_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<WorkoutResponse>> {
        let workouts:Vec<Workout> =workout::table.get_results(c).await?;
        let mut muscle_groups:Vec<MuscleGroup>;
        let mut workout_response:Vec<WorkoutResponse> = Vec::new();
        for workout in workouts {
            muscle_groups = crate::repositories::muscle_groups::MuscleGroupRepository::list_by_workout_id(c, &workout.id).await?;
            workout_response.push(WorkoutResponse {
                id: workout.id,
                workout_name: workout.workout_name,
                notes: workout.notes,
                created_at: workout.created_at,
                muscle_groups,
            });
        }
        return Ok(workout_response);
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_workout: &NewWorkout,
    ) -> QueryResult<Workout> {
        diesel::insert_into(workout::table)
            .values(new_workout)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, workout_item: Workout) -> QueryResult<Workout> {
        diesel::update(workout::table.find(workout_item.id))
            .set((
                workout::workout_name.eq(workout_item.workout_name),
                workout::notes.eq(workout_item.notes),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<usize> {
        diesel::delete(workout::table.find(id)).execute(c).await
    }
}
