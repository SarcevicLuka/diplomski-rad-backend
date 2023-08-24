use async_trait::async_trait;
use error::Error;
use support::store::models::user::DisplayUser;

use super::data::UserInfoResponse;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserContract {
    async fn get_user_by_id(&self, auth_user_id: Option<String>, user_id: &str) -> Result<UserInfoResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_user_by_id(&self, user_id: &str) -> Result<DisplayUser, Error>;
    async fn am_following(&self, auth_user_id: &str, user_id: &str) -> Result<bool, Error>;
}