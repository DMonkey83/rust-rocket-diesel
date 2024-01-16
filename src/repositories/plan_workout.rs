use crate::{
    models::{
        plan_workout::{NewPlanWorkout, PlanWorkout, PlanWorkoutResponse},
        workout::WorkoutResponse,
    },
    schema::planworkout,
};
use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::workout::WorkoutRepository;

pub struct PlanWorkoutRepository;

impl PlanWorkoutRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<PlanWorkoutResponse> {
        let plan_workout: PlanWorkout = planworkout::table.find(id).get_result(c).await?;
        let workout: WorkoutResponse = WorkoutRepository::find(c, &plan_workout.workout_id).await?;
        return Ok(PlanWorkoutResponse {
            id: plan_workout.id,
            workout_id: plan_workout.workout_id,
            plan_id: plan_workout.plan_id,
            workout_day: plan_workout.workout_day,
            notes: plan_workout.notes,
            workout_name: workout.workout_name,
            workout_notes: workout.notes,
            created_at: plan_workout.created_at,
            muscle_groups: workout.muscle_groups,
        });
    }

    pub async fn list_workouts_per_plan(
        c: &mut AsyncPgConnection,
        plan_id: i64,
    ) -> QueryResult<Vec<PlanWorkoutResponse>> {
        let plan_workout: Vec<PlanWorkout> =
            QueryDsl::filter(planworkout::table, planworkout::plan_id.eq(plan_id))
                .get_results(c)
                .await?;
        let mut workout: WorkoutResponse;
        let mut plan_workout_response: Vec<PlanWorkoutResponse> = Vec::new();
        for plan_workout in plan_workout {
            workout = WorkoutRepository::find(c, &plan_workout.workout_id).await?;
            plan_workout_response.push(PlanWorkoutResponse {
                id: plan_workout.id,
                workout_id: plan_workout.workout_id,
                plan_id: plan_workout.plan_id,
                workout_day: plan_workout.workout_day,
                notes: plan_workout.notes,
                workout_name: workout.workout_name,
                workout_notes: workout.notes,
                created_at: plan_workout.created_at,
                muscle_groups: workout.muscle_groups,
            });
        }
        return Ok(plan_workout_response);
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_planworkout: &NewPlanWorkout,
    ) -> QueryResult<PlanWorkout> {
        diesel::insert_into(planworkout::table)
            .values(new_planworkout)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        workout: PlanWorkout,
    ) -> QueryResult<PlanWorkout> {
        diesel::update(planworkout::table.find(workout.id))
            .set((
                planworkout::plan_id.eq(workout.plan_id),
                planworkout::workout_day.eq(workout.workout_day),
                planworkout::notes.eq(workout.notes),
            ))
            .get_result(c)
            .await
    }
}
