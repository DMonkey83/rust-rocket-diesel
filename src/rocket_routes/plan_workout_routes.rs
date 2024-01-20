use crate::{
    models::{
        plan_workout::{NewPlanWorkout, PlanWorkout},
        users::User,
    },
    repositories::plan_workout::PlanWorkoutRepository,
    DbConn,
};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use super::{server_error, EditorUser};

#[rocket::get("/planworkouts?<plan_id>")]
pub async fn list_plan_workouts_by_plan_id(
    mut db: Connection<DbConn>,
    plan_id: i64,
    _user: User,
) -> Result<Value, Custom<Value>> {
    PlanWorkoutRepository::list_workouts_per_plan(&mut db, plan_id)
        .await
        .map(|plan_workout| json!(plan_workout))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/planworkouts/<id>")]
pub async fn get_plan_workout(
    mut db: Connection<DbConn>,
    id: i64,
    _user: User,
) -> Result<Value, Custom<Value>> {
    PlanWorkoutRepository::find(&mut db, &id)
        .await
        .map(|plan_workout| json!(plan_workout))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/planworkouts", format = "json", data = "<new_planworkout>")]
pub async fn create_plan_workout(
    mut db: Connection<DbConn>,
    new_planworkout: Json<NewPlanWorkout>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    PlanWorkoutRepository::create(&mut db, &new_planworkout.into_inner())
        .await
        .map(|plan_workout| Custom(Status::Created, json!(plan_workout)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/planworkouts", format = "json", data = "<plan_workout>")]
pub async fn update_plan_workout(
    mut db: Connection<DbConn>,
    plan_workout: Json<PlanWorkout>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    PlanWorkoutRepository::update(&mut db, plan_workout.into_inner())
        .await
        .map(|plan_workout| Custom(Status::Accepted, json!(plan_workout)))
        .map_err(|e| server_error(e.into()))
}
