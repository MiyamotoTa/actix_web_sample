use async_trait::async_trait;
use regex::Regex;
use slog::{error, o, Logger};

use crate::app::error::AppError;
use crate::app::v1::users::model::user::User;
use crate::app::v1::users::repository::user_repository::{UserRepository, UserRepositoryImpl};
use crate::app::AppState;

#[async_trait]
pub(crate) trait UserService {
    async fn create(&self, email: &str, name: &str) -> Result<(), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<User, AppError>;
}

pub(crate) struct UserServiceImpl<'user_service> {
    user_repository: UserRepositoryImpl<'user_service>,
    log: Logger,
}

impl UserServiceImpl<'_> {
    pub(crate) fn new(state: &AppState) -> UserServiceImpl {
        UserServiceImpl {
            user_repository: UserRepositoryImpl::new(&state.pool),
            log: state.log.new(o!("service"=>"UserServiceImpl")),
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl<'_> {
    async fn create(&self, email: &str, name: &str) -> Result<(), AppError> {
        let log = self.log.new(o!("function" => "create"));

        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if !email_regex.is_match(email) {
            let message = "Invalid e-mail format";
            error!(log, "{}", message);
            return Err(AppError::un_processable_entity_error(message));
        }

        self.user_repository
            .create(email, name)
            .await
            .map_err(AppError::db_error)?;
        Ok(())
    }

    async fn find_by_id(&self, id: u64) -> Result<User, AppError> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await
            .map_err(AppError::db_error)?;
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let user = self
            .user_repository
            .find_by_email(email)
            .await
            .map_err(AppError::db_error)?;
        Ok(user)
    }
}
