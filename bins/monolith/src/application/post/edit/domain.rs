use async_trait::async_trait;
use error::Error;
use super::{
    contract::{EditPostContract, PgRepositoryContract}, 
    data::UserEditPostData
};
use support::store::models::post::{DisplayPost, EditPostData};

pub struct EditPost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> EditPostContract for EditPost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn edit_post(
        &self,
        post_id: &str,
        post_data: UserEditPostData,
    ) -> Result<DisplayPost, Error> {
        let watch_id = self
            .repository
            .get_watch_id(post_id)
            .await?;

        let edit_post_data = EditPostData {
            review: post_data.review.unwrap(),
            score: post_data.score.unwrap()
        };

        let post = self
            .repository
            .edit_post(post_id, edit_post_data)
            .await?;

        let watch = self
            .repository
            .edit_watch(&watch_id, post_data.watch_data)
            .await?;

        Ok(
            DisplayPost {
                id: post.id,
                user_id: post.user_id, 
                watch_data: watch, 
                text: post.review, 
                score: post.score,
                created_at: post.created_at, 
                updated_at: post.updated_at 
            }
        )
    }
}