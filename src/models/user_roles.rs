use diesel::prelude::*;

use crate::schema::user_roles;
use serde::{Deserialize, Serialize};
use super::roles::Role;

#[derive(Queryable, Deserialize, Associations, Serialize)]
#[diesel(belongs_to(Role))]
#[diesel(table_name = user_roles)]
pub struct UserRole {
    pub id: i64,
    pub user_username: String,
    pub role_id: i64,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_username: String,
    pub role_id: i64,
}
