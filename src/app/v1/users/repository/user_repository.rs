use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::app::v1::users::model::user::User;

#[async_trait]
pub(crate) trait UserRepository: Sync {
    async fn create(&self, email: &str, name: &str) -> Result<(), sqlx::Error>;
    async fn find_by_id(&self, id: u64) -> Result<User, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error>;
}

pub(crate) struct UserRepositoryImpl<'user_repository> {
    pool: &'user_repository Pool<MySql>,
}

impl UserRepositoryImpl<'_> {
    pub(crate) fn new(pool: &Pool<MySql>) -> UserRepositoryImpl {
        UserRepositoryImpl { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl<'_> {
    async fn create(&self, email: &str, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query_file!("queries/v1/users/insert_users.sql", name, email)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: u64) -> Result<User, sqlx::Error> {
        let user = sqlx::query_file_as!(User, "queries/v1/users/find_users_by_id.sql", id)
            .fetch_one(self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_file_as!(User, "queries/v1/users/find_users_by_email.sql", email)
            .fetch_one(self.pool)
            .await?;
        Ok(user)
    }
}
