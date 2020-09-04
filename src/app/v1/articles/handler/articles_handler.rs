use crate::app::v1::articles::dto::article_response::ArticleResponse;
use actix_web::{web, HttpRequest, Responder};

pub(crate) async fn index(req: HttpRequest) -> impl Responder {
    let article1 = ArticleResponse {
        title: "title11".to_string(),
        author_name: "author_name1".to_string(),
    };
    let article2 = ArticleResponse {
        title: "title2".to_string(),
        author_name: "author_name2".to_string(),
    };
    let articles = [article1, article2];
    web::Json(articles)
}
