use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::roles;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct Role {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub code: String,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}
