use async_trait::async_trait;
use error::Error;
use super::data::UserLikes;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserLikesContract {
    async fn get_user_likes(&self, user_id: &str) -> Result<UserLikes, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_user_likes(&self, user_id: &str) -> Result<UserLikes, Error>;
}