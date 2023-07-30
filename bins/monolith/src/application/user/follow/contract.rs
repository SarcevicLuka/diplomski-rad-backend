use async_trait::async_trait;
use error::Error;
use support::store::models::user_follow::UserFollow;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FollowUserContract {
    async fn follow_user(&self, request_user_id: &str, response_user_id: &str) -> Result<UserFollow, Error>;
    async fn unfollow_user(&self, user_id: &str, unfollowed_user_id: &str) -> Result<(), Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn follow_user(&self, request_user_id: &str, response_user_id: &str) -> Result<UserFollow, Error>;
    async fn unfollow_user(&self, user_id: &str, unfollowed_user_id: &str) -> Result<usize, Error>;
}