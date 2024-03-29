extern crate workouts;
use rocket_db_pools::Database;
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/api/",
            rocket::routes![
                workouts::rocket_routes::authorization::login,
                workouts::rocket_routes::users_routes::get_user,
                workouts::rocket_routes::users_routes::create_user,
                workouts::rocket_routes::users_routes::delete_user,
                workouts::rocket_routes::user_profile_routes::get_user_profile,
                workouts::rocket_routes::user_profile_routes::create_user_profile,
                workouts::rocket_routes::user_profile_routes::delete_user_profile,
                workouts::rocket_routes::user_profile_routes::update_user_profile,
                workouts::rocket_routes::workout_routes::create_workout,
                workouts::rocket_routes::workout_routes::update_workout,
                workouts::rocket_routes::workout_routes::get_workout,
                workouts::rocket_routes::workout_routes::list_workouts,
                workouts::rocket_routes::workout_routes::delete_workout,
                workouts::rocket_routes::muscle_group_routes::list_muscle_groups_by_workout_id,
                workouts::rocket_routes::muscle_group_routes::create_muscle_group,
                workouts::rocket_routes::muscle_group_routes::update_muscle_gropu,
                workouts::rocket_routes::muscle_group_routes::delete_muscle_group,
                workouts::rocket_routes::plan_workout_routes::list_plan_workouts_by_plan_id,
                workouts::rocket_routes::plan_workout_routes::get_plan_workout,
                workouts::rocket_routes::plan_workout_routes::create_plan_workout,
                workouts::rocket_routes::plan_workout_routes::update_plan_workout,
                workouts::rocket_routes::workout_plan_routes::create_workout_plan,
                workouts::rocket_routes::workout_plan_routes::get_workout_plan,
                workouts::rocket_routes::workout_plan_routes::update_workout_plan,
                workouts::rocket_routes::workout_plan_routes::delete_workout_plan,
                workouts::rocket_routes::exercise_routes::list_exercises,
                workouts::rocket_routes::exercise_routes::find_exercise,
                workouts::rocket_routes::exercise_routes::create_exercise,
                workouts::rocket_routes::exercise_routes::update_exercise,
                workouts::rocket_routes::exercise_routes::delete_exercise,
                workouts::rocket_routes::role_routes::get_role,
                workouts::rocket_routes::role_routes::list_roles,
                workouts::rocket_routes::role_routes::create_role,
                workouts::rocket_routes::role_routes::update_role,
                workouts::rocket_routes::role_routes::delete_role
            ],
        )
        .attach(workouts::rocket_routes::Cors)
        .attach(workouts::CacheConn::init())
        .attach(workouts::DbConn::init())
        .launch()
        .await;
}
