use crate::auth::authorize_user;
use crate::CacheConn;
use crate::{models::credentials::Credentials, repositories::users::UsersRepository, DbConn};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;

use super::server_error;

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let user = UsersRepository::find(&mut db, &credentials.username.clone())
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                Custom(Status::Unauthorized, json!("Wrong credientials"))
            }
            _ => server_error(e.into()),
        })?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!({"error": "Unauthorized"})))?;
    cache
        .set_ex::<String, String, ()>(
            format!("sessions/{}", session_id),
            user.username.clone(),
            3 * 60 * 60,
        )
        .await
        .map_err(|e| server_error(e.into()))?;
    Ok(json!({ "session_id": session_id, "user": user }))
}
