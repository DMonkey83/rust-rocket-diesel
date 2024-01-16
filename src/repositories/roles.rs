use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::roles::{NewRole, Role},
    schema::roles,
};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn list_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Role>> {
        roles::table.get_results(c).await
    }

    pub async fn find(c: &mut AsyncPgConnection, id: &i64) -> QueryResult<Role> {
        roles::table.find(id).get_result(c).await
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
