use std::ops::Deref;

use actix_web::{get, post, web, HttpResponse, Responder};
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
    let email = request.email.as_str();

    users_service::create(email, request.name.as_str(), state.deref())
        .await
        .map_err(|err| {
            let sub_log = log.new(o!("cause" => err.to_string()));
            crit!(sub_log, "Failed to create user");
            err
        })?;

    let user = users_service::find_by_email(email, state.deref())
        .await
        .map_err(|err| {
            let sub_log = log.new(o!("cause" => err.to_string()));
            crit!(sub_log, "Failed to find user");
            err
        })?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/{id}/")]
pub(crate) async fn find_by_id(
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "user_handler.find_by_id"));
    let user = users_service::find_by_id(path.0, state.deref())
        .await
        .map_err(|err| {
            let sub_log = log.new(o!("cause" => err.to_string()));
            crit!(sub_log, "Failed to find user");
            err
        })?;
    Ok(HttpResponse::Ok().json(user))
}
