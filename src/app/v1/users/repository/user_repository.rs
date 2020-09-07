use diesel::{insert_into, ExpressionMethods, QueryResult, RunQueryDsl};

use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::AppState;

pub(crate) fn insert_user(request: &PostUsersRequest, state: &AppState) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    insert_into(users)
        .values((
            name.eq(request.name.clone()),
            email.eq(request.email.clone()),
        ))
        .execute(&state.pool.get().unwrap())
}
