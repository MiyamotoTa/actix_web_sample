use diesel::QueryResult;

use crate::app::error::error::AppError;
use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::repository::user_repository;
use crate::app::AppState;

pub(crate) fn create_users(
    request: &PostUsersRequest,
    state: &AppState,
) -> Result<usize, AppError> {
    let res = user_repository::insert_user(request, state).map_err(AppError::db_error)?;
    Ok(res)
}
