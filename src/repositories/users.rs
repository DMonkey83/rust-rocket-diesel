use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{schema::users, models::users::{User, NewUser}};

pub struct UsersRepository;

impl UsersRepository {
    pub async fn find(c: &mut AsyncPgConnection, username: &str) -> QueryResult<User> {
        users::table.find(username).get_result(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(c)
            .await
    }
    pub async fn delete(c: &mut AsyncPgConnection, username: &str) -> QueryResult<usize> {
        diesel::delete(users::table.find(username))
            .execute(c)
            .await
    }
}
