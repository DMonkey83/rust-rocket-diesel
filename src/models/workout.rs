use crate::schema::workout;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::muscle_group::MuscleGroup;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Workout {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub workout_name: String,
    pub notes: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = workout)]
pub struct NewWorkout {
    pub workout_name: String,
    pub notes: String,
}

#[derive(Serialize, Deserialize)]
pub struct WorkoutResponse {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub workout_name: String,
    pub notes: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    pub muscle_groups: Vec<MuscleGroup>,
}
