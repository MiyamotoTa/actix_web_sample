use actix_web::{Error, HttpRequest, HttpResponse, Responder, ResponseError};
use futures::future::Ready;

use crate::app::error::error::AppError;

pub(crate) async fn handler(req: HttpRequest) -> impl Responder {
    let message = format!(
        "The requested `[{}] {}` was not found.",
        req.method(),
        req.uri().to_string()
    );
    AppError::not_found_error(message)
}

impl Responder for AppError {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        self.error_response().respond_to(_req)
    }
}
