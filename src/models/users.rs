use crate::schema::users;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub password_changed_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}
