use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::{
    models::workout::{NewWorkout, Workout},
    repositories::workout::WorkoutRepository,
    DbConn,
};

#[rocket::get("/workouts/<id>")]
pub async fn get_workout(mut db: Connection<DbConn>, id: i64) -> Result<Value, Custom<Value>> {
    WorkoutRepository::find(&mut db, &id)
        .await
        .map(|workout| json!(workout))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Workout not found, error"),
            )
        })
}

#[rocket::get("/workouts")]
pub async fn list_workouts(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    WorkoutRepository::find_all(&mut db)
        .await
        .map(|workout| json!(workout))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Workout not found, error"),
            )
        })
}

#[rocket::post("/workouts", format = "json", data = "<new_workout>")]
pub async fn create_workout(
    mut db: Connection<DbConn>,
    new_workout: Json<NewWorkout>,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutRepository::create(&mut db, &new_workout.into_inner())
        .await
        .map(|workout| Custom(Status::Created, json!(workout)))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Workout not created, error"),
            )
        })
}

#[rocket::put("/workouts", format = "json", data = "<workout>")]
pub async fn update_workout(
    mut db: Connection<DbConn>,
    workout: Json<Workout>,
) -> Result<Custom<Value>, Custom<Value>> {
    WorkoutRepository::update(&mut db, workout.into_inner())
        .await
        .map(|user_profile| Custom(Status::Accepted, json!(user_profile)))
        .map_err(|_| Custom(Status::InternalServerError, json!("User not found, error")))
}

#[rocket::delete("/workouts/<id>")]
pub async fn delete_workout(mut db: Connection<DbConn>, id: i64) -> Result<Value, Custom<Value>> {
    WorkoutRepository::delete(&mut db, &id)
        .await
        .map(|workout| json!(workout))
        .map_err(|_| Custom(Status::InternalServerError, json!("User not found, error")))
}
