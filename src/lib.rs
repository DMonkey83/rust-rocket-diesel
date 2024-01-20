mod auth;
mod models;
mod repositories;
pub mod rocket_routes;
mod schema;

use rocket_db_pools::{deadpool_redis, diesel, Database};
#[derive(Database)]
#[database("postgres")]
pub struct DbConn(diesel::PgPool);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);
