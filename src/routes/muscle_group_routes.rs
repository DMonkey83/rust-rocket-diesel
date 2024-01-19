use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::models::users::User;
use crate::{
    models::muscle_group::{MuscleGroup, NewMuscleGroup},
    repositories::muscle_groups::MuscleGroupRepository,
    DbConn,
};

use super::server_error;

#[rocket::get("/musclegroups/<workout_id>")]
pub async fn list_muscle_groups_by_workout_id(
    mut db: Connection<DbConn>,
    workout_id: i64,
    _user: User,
) -> Result<Value, Custom<Value>> {
    MuscleGroupRepository::list_by_workout_id(&mut db, &workout_id)
        .await
        .map(|muscle_group| json!(muscle_group))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/musclegroups", format = "json", data = "<new_muscle_group>")]
pub async fn create_muscle_group(
    mut db: Connection<DbConn>,
    new_muscle_group: Json<NewMuscleGroup>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    MuscleGroupRepository::create(&mut db, new_muscle_group.into_inner())
        .await
        .map(|muscle_group| Custom(Status::Created, json!(muscle_group)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/musclegroups", format = "json", data = "<muscle_group>")]
pub async fn update_muscle_gropu(
    mut db: Connection<DbConn>,
    muscle_group: Json<MuscleGroup>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    MuscleGroupRepository::update(&mut db, muscle_group.into_inner())
        .await
        .map(|muscle_group| Custom(Status::Accepted, json!(muscle_group)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/muscle_group/<id>")]
pub async fn delete_muscle_group(
    mut db: Connection<DbConn>,
    id: i64,
    _user: User,
) -> Result<Value, Custom<Value>> {
    MuscleGroupRepository::delete(&mut db, &id)
        .await
        .map(|muscle_group| json!(muscle_group))
        .map_err(|e| server_error(e.into()))
}
