use diesel::{QueryDsl, QueryResult, ExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::workout_plan::{NewWorkoutPlan, WorkoutPlan, WorkoutPlanResponse},
    schema::workoutplan,
};

pub struct WorkoutPlanRepository;

impl WorkoutPlanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<WorkoutPlanResponse> {
        let w_plan: WorkoutPlan = workoutplan::table.find(id).get_result(c).await?;
        let plan_workouts =
            crate::repositories::plan_workout::PlanWorkoutRepository::list_workouts_per_plan(
                c, w_plan.id,
            )
            .await?;
        return Ok(WorkoutPlanResponse {
            id: w_plan.id,
            username: w_plan.username,
            goal: w_plan.goal,
            end_date: w_plan.end_date,
            plan_name: w_plan.plan_name,
            start_date: w_plan.start_date,
            is_public: w_plan.is_public,
            difficulty: w_plan.difficulty,
            description: w_plan.description,
            created_at: w_plan.created_at,
            plan_workouts,
        });
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_workoutplan: &NewWorkoutPlan,
    ) -> QueryResult<WorkoutPlan> {
        diesel::insert_into(workoutplan::table)
            .values(new_workoutplan)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        workout_plan: WorkoutPlan,
    ) -> QueryResult<WorkoutPlan> {
        diesel::update(workoutplan::table.find(workout_plan.id))
            .set((
                workoutplan::plan_name.eq(workout_plan.plan_name),
                workoutplan::description.eq(workout_plan.description),
                workoutplan::start_date.eq(workout_plan.start_date),
                workoutplan::end_date.eq(workout_plan.end_date),
                workoutplan::goal.eq(workout_plan.goal),
                workoutplan::difficulty.eq(workout_plan.difficulty),
                workoutplan::is_public.eq(workout_plan.is_public),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<usize> {
        diesel::delete(workoutplan::table.find(id)).execute(c).await
    }
}
