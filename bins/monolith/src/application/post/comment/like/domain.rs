use async_trait::async_trait;
use error::Error;
use super::contract::{LikeCommentContract, PgRepositoryContract};
use support::store::models::comment_like::CommentLike;

pub struct LikeComment<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> LikeCommentContract for LikeComment<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn like_comment(
        &self,
        user_id: &str,
        comment_id: &str,
    ) -> Result<CommentLike, Error> {
        let comment_like = self
            .repository
            .like_comment(user_id, comment_id)
            .await?;

        Ok(comment_like)
    }

    async fn remove_like_comment(
        &self,
        user_id: &str,
        comment_id: &str,
    ) -> Result<(), Error> {
        self
            .repository
            .remove_like_comment(user_id, comment_id)
            .await?;

        Ok(())
    }
}