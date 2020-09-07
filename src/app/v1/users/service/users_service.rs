use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::model::user::User;
use crate::app::v1::users::repository::user_repository;
use crate::app::AppState;
use diesel::QueryResult;

pub(crate) fn create_users(request: &PostUsersRequest, state: &AppState) -> QueryResult<usize> {
    user_repository::insert_user(request, state)
}
