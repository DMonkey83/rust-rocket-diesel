use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::muscle_group::{MuscleGroup, NewMuscleGroup},
    schema::musclegroups,
};

pub struct MuscleGroupRepository;

impl MuscleGroupRepository {
    pub async fn list_by_workout_id(
        c: &mut AsyncPgConnection,
        workout_id: &i64,
    ) -> QueryResult<Vec<MuscleGroup>> {
        QueryDsl::filter(musclegroups::table, musclegroups::workout_id.eq(workout_id))
            .get_results(c)
            .await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_musclegroup: NewMuscleGroup,
    ) -> QueryResult<MuscleGroup> {
        diesel::insert_into(musclegroups::table)
            .values(new_musclegroup)
            .get_result(c)
            .await
    }
    pub async fn update(
        c: &mut AsyncPgConnection,
        musclegroup: MuscleGroup,
    ) -> QueryResult<MuscleGroup> {
        diesel::update(musclegroups::table.find(musclegroup.id))
            .set((
                musclegroups::name.eq(musclegroup.name),
                musclegroups::workout_id.eq(musclegroup.workout_id),
                musclegroups::muscle_group.eq(musclegroup.muscle_group),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<usize> {
        diesel::delete(musclegroups::table.find(id))
            .execute(c)
            .await
    }
}
