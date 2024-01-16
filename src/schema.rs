// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "difficultyenum"))]
    pub struct Difficultyenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "equipmentenum"))]
    pub struct Equipmentenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "fatiguelevelenum"))]
    pub struct Fatiguelevelenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "genderenum"))]
    pub struct Genderenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "heightenum"))]
    pub struct Heightenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "musclegroupenum"))]
    pub struct Musclegroupenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ratingenum"))]
    pub struct Ratingenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "visibilityenum"))]
    pub struct Visibilityenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "weightenum"))]
    pub struct Weightenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "workoutdayenum"))]
    pub struct Workoutdayenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "workoutgoalenum"))]
    pub struct Workoutgoalenum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Equipmentenum;
    use super::sql_types::Musclegroupenum;

    exercise (exercise_name) {
        #[max_length = 255]
        exercise_name -> Varchar,
        equipment_required -> Equipmentenum,
        description -> Text,
        instructions -> Text,
        muscle_group_name -> Musclegroupenum,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    exerciselog (id) {
        id -> Int8,
        log_id -> Int8,
        #[max_length = 255]
        exercise_name -> Varchar,
        sets_completed -> Int4,
        repetitions_completed -> Int4,
        weight_lifted -> Int4,
        #[max_length = 255]
        notes -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    maxrepgoal (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        exercise_name -> Varchar,
        goal_reps -> Int4,
        notes -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    maxweightgoal (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        exercise_name -> Varchar,
        goal_weight -> Int4,
        notes -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Musclegroupenum;

    musclegroups (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        workout_id -> Int8,
        muscle_group -> Musclegroupenum,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Workoutdayenum;

    planworkout (id) {
        id -> Int8,
        plan_id -> Int8,
        workout_id -> Int8,
        workout_day -> Workoutdayenum,
        notes -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Int8,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    set (id) {
        id -> Int8,
        exercise_log_id -> Int8,
        set_number -> Int4,
        weight -> Int4,
        #[max_length = 8]
        rest_duration -> Varchar,
        reps_completed -> Int4,
        notes -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int8,
        #[max_length = 255]
        user_username -> Varchar,
        role_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Genderenum;
    use super::sql_types::Weightenum;
    use super::sql_types::Heightenum;

    userprofile (username) {
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        age -> Int4,
        gender -> Genderenum,
        height -> Int4,
        preferred_weight_unit -> Weightenum,
        preferred_height_unit -> Heightenum,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (username) {
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        password_changed_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    weightentry (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        entry_date -> Timestamptz,
        weight -> Int4,
        notes -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    workout (id) {
        id -> Int8,
        #[max_length = 255]
        workout_name -> Varchar,
        notes -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    workoutexercises (id) {
        id -> Int8,
        workout_id -> Int8,
        #[max_length = 255]
        exercise_name -> Varchar,
        sets -> Int4,
        #[max_length = 255]
        reps -> Varchar,
        rest_duration -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Ratingenum;
    use super::sql_types::Fatiguelevelenum;

    workoutlog (id) {
        id -> Int8,
        username -> Varchar,
        plan_id -> Int8,
        workout_id -> Int8,
        log_date -> Timestamptz,
        rating -> Ratingenum,
        fatigue_level -> Fatiguelevelenum,
        overall_feeling -> Text,
        comments -> Text,
        #[max_length = 10]
        workout_duration -> Varchar,
        total_calories_burned -> Int4,
        total_distance -> Int4,
        total_repetitions -> Int4,
        total_sets -> Int4,
        total_weight_lifted -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Workoutgoalenum;
    use super::sql_types::Difficultyenum;
    use super::sql_types::Visibilityenum;

    workoutplan (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        plan_name -> Varchar,
        description -> Text,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        goal -> Workoutgoalenum,
        difficulty -> Difficultyenum,
        is_public -> Visibilityenum,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(exerciselog -> exercise (exercise_name));
diesel::joinable!(exerciselog -> workoutlog (log_id));
diesel::joinable!(maxrepgoal -> exercise (exercise_name));
diesel::joinable!(maxrepgoal -> users (username));
diesel::joinable!(maxweightgoal -> exercise (exercise_name));
diesel::joinable!(maxweightgoal -> users (username));
diesel::joinable!(planworkout -> workout (workout_id));
diesel::joinable!(planworkout -> workoutplan (plan_id));
diesel::joinable!(set -> exerciselog (exercise_log_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_username));
diesel::joinable!(userprofile -> users (username));
diesel::joinable!(workoutexercises -> exercise (exercise_name));
diesel::joinable!(workoutexercises -> workout (workout_id));
diesel::joinable!(workoutlog -> planworkout (workout_id));
diesel::joinable!(workoutlog -> users (username));
diesel::joinable!(workoutlog -> workoutplan (plan_id));
diesel::joinable!(workoutplan -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    exercise,
    exerciselog,
    maxrepgoal,
    maxweightgoal,
    musclegroups,
    planworkout,
    roles,
    set,
    user_roles,
    userprofile,
    users,
    weightentry,
    workout,
    workoutexercises,
    workoutlog,
    workoutplan,
);
