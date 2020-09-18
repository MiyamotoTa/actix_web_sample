use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}
