use async_trait::async_trait;
use error::Error;
use support::store::models::user::{User, DisplayUser};
use super::data::LoginUserData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait LoginContract {
    async fn get_user_by_email(&self, email: &str) -> Result<User, Error>;
    async fn login(
        &self,
        data: LoginUserData
    ) -> Result<(DisplayUser, String), Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_user_by_email(&self, email: &str) -> Result<User, Error>;
}