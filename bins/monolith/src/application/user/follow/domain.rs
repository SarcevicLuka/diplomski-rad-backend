use async_trait::async_trait;
use error::Error;
use support::store::models::user_follow::UserFollow;
use super::contract::{FollowUserContract, PgRepositoryContract};

pub struct FollowUser<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> FollowUserContract for FollowUser<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn follow_user(
        &self,
        request_user_id: &str,
        response_user_id: &str,
    ) -> Result<UserFollow, Error> {
        if request_user_id == response_user_id {
            return Err(Error::Request("Cannot follow yourself".to_string()));
        }

        let user_follow = self
            .repository
            .follow_user(request_user_id, response_user_id)
            .await?;

        Ok(user_follow)
    }

    async fn unfollow_user(
        &self,
        user_id: &str,
        unfollowed_user_id: &str,
    ) -> Result<(), Error> {
        if user_id == unfollowed_user_id {
            return Err(Error::Request("Cannot unfollow yourself".to_string()));
        }

        let num_of_deleted_rows = self
            .repository
            .unfollow_user(user_id, unfollowed_user_id)
            .await?;

        if num_of_deleted_rows == 0 {
            return Err(Error::Request("Ids are not valid".to_string()));
        }

        Ok(())
    }
}