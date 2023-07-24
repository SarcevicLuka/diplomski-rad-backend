use async_trait::async_trait;
use error::Error;
use support::store::models::user::DisplayUser;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserContract {
    async fn get_user_by_id(&self, user_id: &str) -> Result<DisplayUser, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_user_by_id(&self, user_id: &str) -> Result<DisplayUser, Error>;
}