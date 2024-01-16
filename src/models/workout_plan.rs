use crate::schema::workoutplan;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    enums_types::{Difficultyenum, Visibilityenum, Workoutgoalenum},
    plan_workout::PlanWorkoutResponse,
};

#[derive(Queryable, Serialize, Deserialize)]
pub struct WorkoutPlan {
    pub id: i64,
    pub username: String,
    pub plan_name: String,
    pub description: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub goal: Workoutgoalenum,
    pub difficulty: Difficultyenum,
    pub is_public: Visibilityenum,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = workoutplan)]
pub struct NewWorkoutPlan {
    pub username: String,
    pub plan_name: String,
    pub description: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub goal: Workoutgoalenum,
    pub difficulty: Difficultyenum,
    pub is_public: Visibilityenum,
}

#[derive(Serialize, Deserialize)]
pub struct WorkoutPlanResponse {
    pub id: i64,
    pub username: String,
    pub plan_name: String,
    pub description: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub goal: Workoutgoalenum,
    pub difficulty: Difficultyenum,
    pub is_public: Visibilityenum,
    pub created_at: NaiveDateTime,
    pub plan_workouts: Vec<PlanWorkoutResponse>,
}
