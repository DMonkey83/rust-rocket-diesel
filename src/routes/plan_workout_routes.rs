use crate::{
    models::plan_workout::{NewPlanWorkout, PlanWorkout},
    repositories::plan_workout::PlanWorkoutRepository,
    DbConn,
};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/planworkouts?<plan_id>")]
pub async fn list_plan_workouts_by_plan_id(
    mut db: Connection<DbConn>,
    plan_id: i64,
) -> Result<Value, Custom<Value>> {
    PlanWorkoutRepository::list_workouts_per_plan(&mut db, plan_id)
        .await
        .map(|plan_workout| json!(plan_workout))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Plan Workout not found, error"),
            )
        })
}

#[rocket::get("/planworkouts/<id>")]
pub async fn get_plan_workout(mut db: Connection<DbConn>, id: i64) -> Result<Value, Custom<Value>> {
    PlanWorkoutRepository::find(&mut db, &id)
        .await
        .map(|plan_workout| json!(plan_workout))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Plan Workout not found, error"),
            )
        })
}

#[rocket::post("/planworkouts", format = "json", data = "<new_planworkout>")]
pub async fn create_plan_workout(
    mut db: Connection<DbConn>,
    new_planworkout: Json<NewPlanWorkout>,
) -> Result<Custom<Value>, Custom<Value>> {
    PlanWorkoutRepository::create(&mut db, &new_planworkout.into_inner())
        .await
        .map(|plan_workout| Custom(Status::Created, json!(plan_workout)))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Plan Workout not created, error"),
            )
        })
}

#[rocket::put("/planworkouts", format = "json", data = "<plan_workout>")]
pub async fn update_plan_workout(
    mut db: Connection<DbConn>,
    plan_workout: Json<PlanWorkout>,
) -> Result<Custom<Value>, Custom<Value>> {
    PlanWorkoutRepository::update(&mut db, plan_workout.into_inner())
        .await
        .map(|plan_workout| Custom(Status::Accepted, json!(plan_workout)))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Plan Workout not found, error"),
            )
        })
}
