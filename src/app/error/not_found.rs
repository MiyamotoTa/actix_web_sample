use crate::app::error::error_message::ErrorMessage;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use http::StatusCode;

pub(crate) async fn handler(req: HttpRequest) -> impl Responder {
    let message = format!(
        "The requested `[{}] {}` was not found.",
        req.method(),
        req.uri().to_string()
    );
    ErrorMessage {
        status: StatusCode::NOT_FOUND.as_u16(),
        message,
        code: None,
    }
}

impl Responder for ErrorMessage {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::NotFound()
            .content_type("application/json")
            .body(body)))
    }
}
