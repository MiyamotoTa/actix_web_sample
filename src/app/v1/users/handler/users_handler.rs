use std::ops::Deref;

use actix_web::{get, post, web, HttpResponse, Responder};
use slog::{crit, o};

use crate::app::error::AppError;
use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::service::user_service::{UserService, UserServiceImpl};
use crate::app::AppState;

#[post("/")]
pub(crate) async fn create(
    request: web::Json<PostUsersRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "user_handler.create"));
    let email = request.email.as_str();

    let user_service = UserServiceImpl::new(state.deref());
    user_service
        .create(email, request.name.as_str())
        .await
        .map_err(|err| {
            let sub_log = log.new(o!("cause" => err.to_string()));
            crit!(sub_log, "Failed to create user");
            err
        })?;

    let user = user_service.find_by_email(email).await.map_err(|err| {
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

    let user_service = UserServiceImpl::new(state.deref());
    let user = user_service.find_by_id(path.0).await.map_err(|err| {
        let sub_log = log.new(o!("cause" => err.to_string()));
        crit!(sub_log, "Failed to find user");
        err
    })?;
    Ok(HttpResponse::Ok().json(user))
}
