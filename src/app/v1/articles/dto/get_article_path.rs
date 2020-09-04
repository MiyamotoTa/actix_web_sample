use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GetArticlePath {
    pub(crate) author: String,
    pub(crate) title: String,
}
