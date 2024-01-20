use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{roles::{NewRole, Role}, user_roles::UserRole, users::User},
    schema::{roles, user_roles},
};

use super::users::UsersRepository;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn list_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Role>> {
        roles::table.get_results(c).await
    }

    pub async fn find_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i64>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user = UsersRepository::find(c, &user.username).await?;
        let user_roles = user_roles::table
            .filter(user_roles::user_username.eq(user.username))
            .get_results::<UserRole>(c)
            .await?;
        let role_ids: Vec<i64> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();

        Self::find_by_ids(c, role_ids).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, role: Role) -> QueryResult<Role> {
        diesel::update(roles::table.find(role.id))
            .set((roles::name.eq(role.name), roles::code.eq(role.code)))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<usize> {
        diesel::delete(roles::table.find(id)).execute(c).await
    }
}
