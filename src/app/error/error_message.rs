use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ErrorMessage {
    pub(crate) status: u16,
    pub(crate) message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) code: Option<u16>,
}
