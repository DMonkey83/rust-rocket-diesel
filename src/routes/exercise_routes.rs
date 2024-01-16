use crate::{
    models::exercise::{Exercise, NewExercise},
    repositories::exercise::ExerciseRepository,
    DbConn,
};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/exercises")]
pub async fn list_exercises(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    ExerciseRepository::list_all(&mut db)
        .await
        .map(|exercise| json!(exercise))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Exercise not found, error"),
            )
        })
}

#[rocket::get("/exercises/<exercise_name>")]
pub async fn find_exercise(mut db: Connection<DbConn>, exercise_name: &str) -> Result<Value, Custom<Value>> {
    ExerciseRepository::find(&mut db, exercise_name)
        .await
        .map(|exercise| json!(exercise))
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                json!("Exercise not found, error"),
            )
        })
}

#[rocket::post("/exercises", format = "json", data = "<new_exercise>")]
pub async fn create_exercise(
    mut db: Connection<DbConn>,
    new_exercise: Json<NewExercise>,
) -> Result<Custom<Value>, Custom<Value>> {
    ExerciseRepository::create(&mut db, new_exercise.into_inner())
        .await
        .map(|exercise| Custom(Status::Created, json!(exercise)))
        .map_err(|e| {
            rocket::info!("Error: {}", e);
            Custom(
                Status::InternalServerError,
                json!("Muscle Group not created, error"),
            )
        })
}

#[rocket::put("/exercises", format = "json", data = "<exercise>")]
pub async fn update_exercise(
    mut db: Connection<DbConn>,
    exercise: Json<Exercise>,
) -> Result<Custom<Value>, Custom<Value>> {
    ExerciseRepository::update(&mut db, exercise.into_inner())
        .await
        .map(|exercise| Custom(Status::Accepted, json!(exercise)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Exercise not found, error")))
}

#[rocket::delete("/exercises/<exercise_name>")]
pub async fn delete_exercise(
    mut db: Connection<DbConn>,
    exercise_name: &str,
) -> Result<Value, Custom<Value>> {
    ExerciseRepository::delete(&mut db, exercise_name)
        .await
        .map(|exercise| json!(exercise))
        .map_err(|_| Custom(Status::InternalServerError, json!("Exercise not found, error")))
}
