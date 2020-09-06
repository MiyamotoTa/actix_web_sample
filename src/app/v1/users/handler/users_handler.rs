use crate::app::v1::users::dto::post_users_request::PostUsersRequest;
use actix_web::{post, web, Responder};

#[post("/")]
pub(crate) async fn create_user(user: web::Json<PostUsersRequest>) -> impl Responder {
    format!("Welcome {}!", user.name)
}
