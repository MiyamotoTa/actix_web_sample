use serde::Serialize;

#[derive(Serialize)]
pub struct ArticleResponse {
    pub title: String,
    pub author_name: String,
}
