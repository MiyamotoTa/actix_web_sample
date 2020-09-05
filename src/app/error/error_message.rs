use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ErrorMessage {
    pub(crate) status: u16,
    pub(crate) message: String,
    pub(crate) code: Option<u16>,
}
