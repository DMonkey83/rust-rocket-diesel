use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::roles;
use serde::{Deserialize, Serialize};

use super::enums_types::RoleCode;

#[derive(Queryable,Debug, Deserialize, Serialize, Identifiable)]
pub struct Role {
    pub id: i64,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}
