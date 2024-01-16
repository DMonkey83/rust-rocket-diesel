use crate::schema::musclegroups;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums_types::Musclegroupenum;


#[derive(AsChangeset, Queryable, Deserialize, Serialize)]
#[diesel(table_name = musclegroups)]
pub struct MuscleGroup {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub workout_id: i64,
    pub muscle_group: Musclegroupenum,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = musclegroups)]
pub struct NewMuscleGroup {
    pub name: String,
    pub workout_id: i64,
    pub muscle_group: Musclegroupenum,
}
