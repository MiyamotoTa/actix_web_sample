use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PostUsersRequest {
    pub(crate) email: String,
    pub(crate) name: String,
}
