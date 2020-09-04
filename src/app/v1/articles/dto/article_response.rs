use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ArticleResponse {
    pub title: String,
    pub author_name: String,
}
