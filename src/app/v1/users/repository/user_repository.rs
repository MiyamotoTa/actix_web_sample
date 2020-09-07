use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::model::user::User;
use crate::app::AppState;
use crate::schema::users::dsl::users;
use diesel::sql_types::Varchar;
use diesel::{
    insert_into, sql_query, ExpressionMethods, MysqlConnection, QueryResult, RunQueryDsl,
};

pub(crate) fn insert_user(request: &PostUsersRequest, state: &AppState) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    insert_into(users)
        .values((
            name.eq(request.name.clone()),
            email.eq(request.email.clone()),
        ))
        .execute(&state.pool.get().unwrap())
}
