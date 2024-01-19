use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Value};
use rocket::Request;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use std::error::Error;

use crate::models::users::User;
use crate::repositories::users::UsersRepository;
use crate::CacheConn;
use crate::DbConn;

pub mod authorization;
pub mod exercise_routes;
pub mod muscle_group_routes;
pub mod plan_workout_routes;
pub mod role_routes;
pub mod user_profile_routes;
pub mod users_routes;
pub mod workout_plan_routes;
pub mod workout_routes;

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Internal Server Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Custom<Value>;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");
        if let Some(header_value) = session_header {
            let mut cache = req
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Cache connection failed");
            let mut db = req
                .guard::<Connection<DbConn>>()
                .await
                .expect("Database connection failed");
            let result = cache
                .get::<String, String>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(username) = result {
               if let Ok(user) = UsersRepository::find(&mut db, &username)
                    .await{
                        return Outcome::Success(user);
                }
                               }
        }
        Outcome::Error((Status::Unauthorized, Custom(Status::Unauthorized, json!({"error": "Unauthorized"}))))
    }
}
