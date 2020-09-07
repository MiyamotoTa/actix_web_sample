use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use crate::app::v1::users::service::users_service;
use crate::app::AppState;
use actix_web::{post, web, Responder};
use std::ops::Deref;

#[post("/")]
pub(crate) async fn create_user(
    request: web::Json<PostUsersRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let user = users_service::create_users(request.deref(), state.deref());
    format!("{}", "inserted")
}
