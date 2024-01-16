use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::{models::users::NewUser, repositories::users::UsersRepository, DbConn};

#[rocket::get("/users/<username>")]
pub async fn get_user(mut db: Connection<DbConn>, username: &str) -> Result<Value, Custom<Value>> {
    UsersRepository::find(&mut db, username)
        .await
        .map(|user| json!(user))
        .map_err(|_| Custom(Status::InternalServerError, json!("User not found, error")))
}

#[rocket::post("/users", format = "json", data = "<new_user>")]
pub async fn create_user(
    mut db: Connection<DbConn>,
    new_user: Json<NewUser>,
) -> Result<Custom<Value>, Custom<Value>> {
    UsersRepository::create(&mut db, &new_user.into_inner())
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                json!(format!("User not found, error{e}")),
            )
        })
}

#[rocket::delete("/users/<username>")]
pub async fn delete_user(
    mut db: Connection<DbConn>,
    username: &str,
) -> Result<Value, Custom<Value>> {
    UsersRepository::delete(&mut db, username)
        .await
        .map(|user| json!(user))
        .map_err(|_| Custom(Status::InternalServerError, json!("User not found, error")))
}
