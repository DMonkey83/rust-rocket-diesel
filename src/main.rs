use routes::{
    muscle_group_routes::{
        create_muscle_group, delete_muscle_group, list_muscle_groups_by_workout_id,
        update_muscle_gropu,
    },
    plan_workout_routes::{
        create_plan_workout, get_plan_workout, list_plan_workouts_by_plan_id, update_plan_workout,
    },
    user_profile_routes::{
        create_user_profile, delete_user_profile, get_user_profile, update_user_profile,
    },
    users_routes::{create_user, delete_user, get_user},
    workout_plan_routes::{
        create_workout_plan, delete_workout_plan, get_workout_plan, update_workout_plan,
    },
    workout_routes::{create_workout, delete_workout, get_workout, list_workouts, update_workout}, exercise_routes::{list_exercises, find_exercise, create_exercise, update_exercise, delete_exercise},
    role_routes::{get_role, list_roles, create_role, update_role, delete_role}, authorization::login,
};

mod auth;
mod models;
mod repositories;
mod routes;
mod schema;

use rocket_db_pools::{Database, diesel, deadpool_redis};
#[derive(Database)]
#[database("postgres")]
pub struct DbConn(diesel::PgPool);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                login,
                get_user,
                create_user,
                delete_user,
                get_user_profile,
                create_user_profile,
                delete_user_profile,
                update_user_profile,
                create_workout,
                update_workout,
                get_workout,
                list_workouts,
                delete_workout,
                list_muscle_groups_by_workout_id,
                create_muscle_group,
                update_muscle_gropu,
                delete_muscle_group,
                list_plan_workouts_by_plan_id,
                get_plan_workout,
                create_plan_workout,
                update_plan_workout,
                create_workout_plan,
                get_workout_plan,
                update_workout_plan,
                delete_workout_plan,
                list_exercises,
                find_exercise,
                create_exercise,
                update_exercise,
                delete_exercise,
                get_role,
                list_roles,
                create_role,
                update_role,
                delete_role
            ],
        )
        .attach(CacheConn::init())
        .attach(DbConn::init())
        .launch()
        .await;
}
