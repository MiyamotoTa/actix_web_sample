use crate::app::error::AppError;
use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::model::user::User;
use crate::app::v1::users::repository::user_repository;
use crate::app::AppState;

pub(crate) async fn create(request: &PostUsersRequest, state: &AppState) -> Result<(), AppError> {
    user_repository::create(request, state)
        .await
        .map_err(AppError::db_error)?;
    Ok(())
}

pub(crate) async fn find_by_email(email: &str, state: &AppState) -> Result<User, AppError> {
    let user = user_repository::find_by_email(email, state)
        .await
        .map_err(AppError::db_error)?;
    Ok(user)
}
