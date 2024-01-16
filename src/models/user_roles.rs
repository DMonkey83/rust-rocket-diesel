use diesel::prelude::*;

use crate::schema::user_roles;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
pub struct UserRole {
    pub id: i64,
    pub user_username: String,
    pub role_id: i64,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_roles)]
pub struct NewRole {
    pub user_username: String,
    pub role_id: i64,
}
