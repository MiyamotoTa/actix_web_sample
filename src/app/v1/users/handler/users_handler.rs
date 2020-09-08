use std::ops::Deref;

use actix_web::{post, web, HttpResponse, Responder};
use slog::{crit, o};

use crate::app::error::error::AppError;
use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::service::users_service;
use crate::app::AppState;

#[post("/")]
pub(crate) async fn create_user(
    request: web::Json<PostUsersRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "create_user"));
    users_service::create_users(request.deref(), state.deref())
        .await
        .map_err(|err| {
            let sublog = log.new(o!("cause"=>err.to_string()));
            crit!(sublog, "Error creating");
            AppError::db_error(err)
        })?;

    Ok(HttpResponse::Ok().json("{ok:ok}"))
}
