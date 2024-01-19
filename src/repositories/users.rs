use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{
        roles::NewRole,
        user_roles::{NewUserRole, UserRole},
        users::{NewUser, User},
    },
    schema::{user_roles, users},
};

use super::roles::RoleRepository;

pub struct UsersRepository;

impl UsersRepository {
    pub async fn find(c: &mut AsyncPgConnection, username: &str) -> QueryResult<User> {
        users::table.find(username).get_result(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_user: &NewUser,
        role_codes: &Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)
            .await?;
        let username = &user.username;
        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code.to_owned()).await {
                    NewUserRole {
                        user_username: username.clone(),
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        name: role_code.to_owned(),
                        code: role_code.to_owned(),
                    };
                    let role = RoleRepository::create(c, new_role).await?;
                    NewUserRole {
                        user_username: username.clone(),
                        role_id: role.id,
                    }
                }
            };
            diesel::insert_into(user_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)
                .await?;
        }
        Ok(user)
    }
    pub async fn delete(c: &mut AsyncPgConnection, username: &str) -> QueryResult<usize> {
        diesel::delete(users::table.find(username)).execute(c).await
    }
}
