use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub(crate) struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}
