use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::models::users::User;
use crate::{
    models::workout::{NewWorkout, Workout},
    repositories::workout::WorkoutRepository,
    DbConn,
};

use super::server_error;

#[rocket::get("/workouts/<id>")]
pub async fn get_workout(
    mut db: Connection<DbConn>,
    id: i64,
    _user: User,
) -> Result<Value, Custom<Value>> {
    WorkoutRepository::find(&mut db, &id)
        .await
        .map(|workout| json!(workout))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/workouts")]
pub async fn list_workouts(
    mut db: Connection<DbConn>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    WorkoutRepository::find_all(&mut db)
        .await
        .map(|workout| json!(workout))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/workouts", format = "json", data = "<new_workout>")]
pub async fn create_workout(
    mut db: Connection<DbConn>,
    new_workout: Json<NewWorkout>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutRepository::create(&mut db, &new_workout.into_inner())
        .await
        .map(|workout| Custom(Status::Created, json!(workout)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/workouts", format = "json", data = "<workout>")]
pub async fn update_workout(
    mut db: Connection<DbConn>,
    workout: Json<Workout>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutRepository::update(&mut db, workout.into_inner())
        .await
        .map(|user_profile| Custom(Status::Accepted, json!(user_profile)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/workouts/<id>")]
pub async fn delete_workout(
    mut db: Connection<DbConn>,
    id: i64,

    _user: User,
) -> Result<Value, Custom<Value>> {
    WorkoutRepository::delete(&mut db, &id)
        .await
        .map(|workout| json!(workout))
        .map_err(|e| server_error(e.into()))
}
