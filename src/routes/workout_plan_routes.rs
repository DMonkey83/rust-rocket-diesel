use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::models::workout_plan::WorkoutPlan;
use crate::{
    models::workout_plan::NewWorkoutPlan, repositories::workout_plan::WorkoutPlanRepository, DbConn,
};

#[rocket::get("/workoutplans/<id>")]
pub async fn get_workout_plan(mut db: Connection<DbConn>, id: i64) -> Result<Value, Custom<Value>> {
    WorkoutPlanRepository::find(&mut db, &id)
        .await
        .map(|workout_plan| json!(workout_plan))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
}

#[rocket::post("/workoutplans", format = "json", data = "<new_workout_plan>")]
pub async fn create_workout_plan(
    mut db: Connection<DbConn>,
    new_workout_plan: Json<NewWorkoutPlan>,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutPlanRepository::create(&mut db, &new_workout_plan.into_inner())
        .await
        .map(|workout_plan| Custom(Status::Created, json!(workout_plan)))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
}

#[rocket::put("/workoutplans", format = "json", data = "<workout_plan>")]
pub async fn update_workout_plan(
    mut db: Connection<DbConn>,
    workout_plan: Json<WorkoutPlan>,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutPlanRepository::update(&mut db, workout_plan.into_inner())
        .await
        .map(|workout_plan| Custom(Status::Created, json!(workout_plan)))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
}

#[rocket::delete("/workoutplans/<id>")]
pub async fn delete_workout_plan(
    mut db: Connection<DbConn>,
    id: i64,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutPlanRepository::delete(&mut db, &id)
        .await
        .map(|_| Custom(Status::Ok, json!("Workout Plan deleted")))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
}
