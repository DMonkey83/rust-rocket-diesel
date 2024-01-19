use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::models::users::User;
use crate::{
    models::user_profile::{NewUserProfile, UserProfile},
    repositories::user_profile::UserProfileRepository,
    DbConn,
};

use super::server_error;

#[rocket::get("/user_profile/<username>")]
pub async fn get_user_profile(
    mut db: Connection<DbConn>,
    username: &str,
    _user: User,
) -> Result<Value, Custom<Value>> {
    UserProfileRepository::find(&mut db, username)
        .await
        .map(|user_profile| json!(user_profile))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/user_Profile", format = "json", data = "<new_user_profile>")]
pub async fn create_user_profile(
    mut db: Connection<DbConn>,
    new_user_profile: Json<NewUserProfile>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    UserProfileRepository::create(&mut db, &new_user_profile.into_inner())
        .await
        .map(|user_profile| Custom(Status::Created, json!(user_profile)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/user_Profile", format = "json", data = "<user_profile>")]
pub async fn update_user_profile(
    mut db: Connection<DbConn>,
    user_profile: Json<UserProfile>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    UserProfileRepository::update(&mut db, user_profile.into_inner())
        .await
        .map(|user_profile| Custom(Status::Accepted, json!(user_profile)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/user_profile/<username>")]
pub async fn delete_user_profile(
    mut db: Connection<DbConn>,
    username: &str,
    _user: User,
) -> Result<Value, Custom<Value>> {
    UserProfileRepository::delete(&mut db, username)
        .await
        .map(|user_profile| json!(user_profile))
        .map_err(|e| server_error(e.into()))
}
