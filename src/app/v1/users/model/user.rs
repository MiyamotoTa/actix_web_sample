use chrono::NaiveDateTime;
use diesel::deserialize::{QueryableByName, Result};
use diesel::mysql::types::Datetime;
use diesel::row::NamedRow;
use serde::Serialize;

type DB = diesel::mysql::Mysql;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub(crate) struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

impl QueryableByName<DB> for User {
    fn build<R: NamedRow<DB>>(row: &R) -> Result<Self> {
        Ok(User {
            id: row.get("id")?,
            name: row.get("name")?,
            email: row.get("email")?,
            created_at: row.get::<Datetime, NaiveDateTime>("created_at")?,
            updated_at: row.get::<Datetime, NaiveDateTime>("updated_at")?,
        })
    }
}
