use std::ops::Deref;

use actix_web::{post, web, HttpResponse, Responder};
use slog::{crit, o};

use crate::app::error::AppError;
use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::service::users_service;
use crate::app::AppState;

#[post("/")]
pub(crate) async fn create(
    request: web::Json<PostUsersRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "user_handler.create"));
    users_service::create(request.deref(), state.deref())
        .await
        .map_err(|err| {
            let sub_log = log.new(o!("cause" => err.to_string()));
            crit!(sub_log, "Failed to create user");
            AppError::db_error(err)
        })?;

    let user = users_service::find_by_email(request.email.as_str(), state.deref())
        .await
        .map_err(|err| {
            let sub_log = log.new(o!("cause" => err.to_string()));
            crit!(sub_log, "Failed to find user");
            AppError::db_error(err)
        })?;
    Ok(HttpResponse::Ok().json(user))
}
