use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::exercise::{Exercise, NewExercise},
    schema::exercise,
};

pub struct ExerciseRepository;

impl ExerciseRepository {
    pub async fn list_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Exercise>> {
        exercise::table.get_results(c).await
    }

    pub async fn find(c: &mut AsyncPgConnection, exercise_name: &str) -> QueryResult<Exercise> {
        exercise::table.find(exercise_name).get_result(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_exercise: NewExercise,
    ) -> QueryResult<Exercise> {
        diesel::insert_into(exercise::table)
            .values(new_exercise)
            .get_result(c)
            .await
    }
    pub async fn update(
        c: &mut AsyncPgConnection,
        exercise: Exercise,
    ) -> QueryResult<Exercise> {
        diesel::update(exercise::table.find(exercise.exercise_name))
            .set((
                exercise::instructions.eq(exercise.instructions),
                exercise::description.eq(exercise.description),
                exercise::equipment_required.eq(exercise.equipment_required),
                exercise::muscle_group_name.eq(exercise.muscle_group_name),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, exercise_name: &str) -> QueryResult<usize> {
        diesel::delete(exercise::table.find(exercise_name))
            .execute(c)
            .await
    }
}
