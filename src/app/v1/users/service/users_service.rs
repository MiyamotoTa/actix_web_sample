use crate::app::error::error::AppError;
use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::repository::user_repository;
use crate::app::AppState;

pub(crate) async fn create_users(
    request: &PostUsersRequest,
    state: &AppState,
) -> Result<(), AppError> {
    let res = user_repository::insert_user(request, state)
        .await
        .map_err(AppError::db_error)?;
    Ok(res)
}
