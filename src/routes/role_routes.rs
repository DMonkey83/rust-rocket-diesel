use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::models::users::User;
use crate::{
    models::roles::{NewRole, Role},
    repositories::roles::RoleRepository,
    DbConn,
};

use super::server_error;

#[rocket::get("/roles/<code>")]
pub async fn get_role(
    mut db: Connection<DbConn>,
    code: String,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RoleRepository::find_by_code(&mut db, code)
        .await
        .map(|roles| json!(roles))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/roles")]
pub async fn list_roles(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    RoleRepository::list_all(&mut db)
        .await
        .map(|roles| json!(roles))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/roles", format = "json", data = "<new_role>")]
pub async fn create_role(
    mut db: Connection<DbConn>,
    new_role: Json<NewRole>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    RoleRepository::create(&mut db, new_role.into_inner())
        .await
        .map(|roles| Custom(Status::Created, json!(roles)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/roles", format = "json", data = "<role>")]
pub async fn update_role(
    mut db: Connection<DbConn>,
    role: Json<Role>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    RoleRepository::update(&mut db, role.into_inner())
        .await
        .map(|roles| Custom(Status::Accepted, json!(roles)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/roles/<id>")]
pub async fn delete_role(
    mut db: Connection<DbConn>,
    id: i64,
    _user: User,
) -> Result<Value, Custom<Value>> {
    RoleRepository::delete(&mut db, &id)
        .await
        .map(|roles| json!(roles))
        .map_err(|e| server_error(e.into()))
}
