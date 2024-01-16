use crate::schema::planworkout;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

use super::{enums_types::Workoutdayenum, muscle_group::MuscleGroup};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct PlanWorkout {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub plan_id: i64,
    pub workout_id: i64,
    pub workout_day: Workoutdayenum,
    pub notes: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = planworkout)]
pub struct NewPlanWorkout {
    pub plan_id: i64,
    pub workout_id: i64,
    pub workout_day: Workoutdayenum,
    pub notes: String,
}

#[derive(Deserialize, Serialize)]
pub struct PlanWorkoutResponse {
    pub id: i64,
    pub plan_id: i64,
    pub workout_id: i64,
    pub workout_name: String,
    pub workout_day: Workoutdayenum,
    pub notes: String,
    pub created_at: NaiveDateTime,
    pub workout_notes: String,
    pub muscle_groups: Vec<MuscleGroup>,
}
