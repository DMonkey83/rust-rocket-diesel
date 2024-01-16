use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::user_profile::{NewUserProfile, UserProfile},
    schema::userprofile,
};

pub struct UserProfileRepository;

impl UserProfileRepository {
    pub async fn find(c: &mut AsyncPgConnection, username: &str) -> QueryResult<UserProfile> {
        userprofile::table.find(username).get_result(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_userprofile: &NewUserProfile,
    ) -> QueryResult<UserProfile> {
        diesel::insert_into(userprofile::table)
            .values(new_userprofile)
            .get_result(c)
            .await
    }
    pub async fn update(
        c: &mut AsyncPgConnection,
        profile: UserProfile,
    ) -> QueryResult<UserProfile> {
        diesel::update(userprofile::table.find(profile.username))
            .set((
                userprofile::first_name.eq(profile.first_name),
                userprofile::last_name.eq(profile.last_name),
                userprofile::email.eq(profile.email),
                userprofile::age.eq(profile.age),
                userprofile::gender.eq(profile.gender),
                userprofile::height.eq(profile.height),
                userprofile::preferred_weight_unit.eq(profile.preferred_weight_unit),
                userprofile::preferred_height_unit.eq(profile.preferred_height_unit),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, username: &str) -> QueryResult<usize> {
        diesel::delete(userprofile::table.find(username))
            .execute(c)
            .await
    }
}
