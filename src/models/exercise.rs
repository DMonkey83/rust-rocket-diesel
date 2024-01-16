use crate::schema::exercise;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums_types::{Equipmentenum, Musclegroupenum};

#[derive(AsChangeset, Queryable, Deserialize, Serialize)]
#[diesel(table_name = exercise)]
pub struct Exercise {
    pub exercise_name: String,
    pub equipment_required: Equipmentenum,
    pub description: String,
    pub instructions: String,
    pub muscle_group_name: Musclegroupenum,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = exercise)]
pub struct NewExercise {
    pub muscle_group_name: Musclegroupenum,
    pub equipment_required: Equipmentenum,
    pub description: String,
    pub instructions: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExerciseItem {
    pub exercise_name: String,
    pub equipment_required: Equipmentenum,
    pub description: String,
    pub instructions: String,
    pub muscle_group_name: Musclegroupenum,
}
