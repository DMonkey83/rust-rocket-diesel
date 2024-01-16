use diesel::prelude::*;

use crate::schema::userprofile;

use super::enums_types::{Genderenum, Heightenum, Weightenum};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub gender: Genderenum,
    pub height: i32,
    pub preferred_weight_unit: Weightenum,
    pub preferred_height_unit: Heightenum,
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = userprofile)]
pub struct NewUserProfile {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub gender: Genderenum,
    pub height: i32,
    pub preferred_weight_unit: Weightenum,
    pub preferred_height_unit: Heightenum,
}
