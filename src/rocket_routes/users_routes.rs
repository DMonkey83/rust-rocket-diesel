use std::str::FromStr;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::auth::hash_password;
use crate::models::enums_types::RoleCode;
use crate::models::users::{NewUser, NewUserData, User};
use crate::{repositories::users::UsersRepository, DbConn};

use super::server_error;

#[rocket::get("/users/<username>")]
pub async fn get_user(
    mut db: Connection<DbConn>,
    username: &str,
    _user: User,
) -> Result<Value, Custom<Value>> {
    UsersRepository::find(&mut db, username)
        .await
        .map(|user| json!(user))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/users", format = "json", data = "<new_user_data>")]
pub async fn create_user(
    mut db: Connection<DbConn>,
    new_user_data: Json<NewUserData>,
) -> Result<Custom<Value>, Custom<Value>> {
    let password_hash = hash_password(new_user_data.new_user.password_hash.clone()).unwrap();
    let new_user = &NewUser {
        username: new_user_data.new_user.username.clone(),
        password_hash: password_hash.clone(),
    };
    let role_enums = new_user_data.role_codes.iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();
    UsersRepository::create(&mut db, &new_user, role_enums)
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/users/<username>")]
pub async fn delete_user(
    mut db: Connection<DbConn>,
    username: &str,
    _user: User,
) -> Result<Value, Custom<Value>> {
    UsersRepository::delete(&mut db, username)
        .await
        .map(|user| json!(user))
        .map_err(|e| server_error(e.into()))
}
