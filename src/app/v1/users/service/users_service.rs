use regex::Regex;
use slog::{error, o};

use crate::app::error::AppError;
use crate::app::v1::users::model::user::User;
use crate::app::v1::users::repository::user_repository;
use crate::app::AppState;

pub(crate) async fn create(email: &str, name: &str, state: &AppState) -> Result<(), AppError> {
    let log = state.log.new(o!("service" => "user_service.create"));

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(email) {
        let message = "Invalid e-mail format";
        error!(log, "{}", message);
        return Err(AppError::un_processable_entity_error(message));
    }

    user_repository::create(email, name, state)
        .await
        .map_err(AppError::db_error)?;
    Ok(())
}

pub(crate) async fn find_by_id(id: u64, state: &AppState) -> Result<User, AppError> {
    let user = user_repository::find_by_id(id, state)
        .await
        .map_err(AppError::db_error)?;
    Ok(user)
}

pub(crate) async fn find_by_email(email: &str, state: &AppState) -> Result<User, AppError> {
    let user = user_repository::find_by_email(email, state)
        .await
        .map_err(AppError::db_error)?;
    Ok(user)
}
