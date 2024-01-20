use crate::{
    models::{
        exercise::{Exercise, NewExercise},
        users::User,
    },
    repositories::exercise::ExerciseRepository,
    DbConn,
};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use super::{server_error, EditorUser};

#[rocket::get("/exercises")]
pub async fn list_exercises(
    mut db: Connection<DbConn>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    ExerciseRepository::list_all(&mut db)
        .await
        .map(|exercise| json!(exercise))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/exercises/<exercise_name>")]
pub async fn find_exercise(
    mut db: Connection<DbConn>,
    exercise_name: &str,
    _user: User,
) -> Result<Value, Custom<Value>> {
    ExerciseRepository::find(&mut db, exercise_name)
        .await
        .map(|exercise| json!(exercise))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/exercises", format = "json", data = "<new_exercise>")]
pub async fn create_exercise(
    mut db: Connection<DbConn>,
    new_exercise: Json<NewExercise>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    ExerciseRepository::create(&mut db, new_exercise.into_inner())
        .await
        .map(|exercise| Custom(Status::Created, json!(exercise)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/exercises", format = "json", data = "<exercise>")]
pub async fn update_exercise(
    mut db: Connection<DbConn>,
    exercise: Json<Exercise>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    ExerciseRepository::update(&mut db, exercise.into_inner())
        .await
        .map(|exercise| Custom(Status::Accepted, json!(exercise)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/exercises/<exercise_name>")]
pub async fn delete_exercise(
    mut db: Connection<DbConn>,
    exercise_name: &str,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    ExerciseRepository::delete(&mut db, exercise_name)
        .await
        .map(|exercise| json!(exercise))
        .map_err(|e| server_error(e.into()))
}
