use async_trait::async_trait;
use error::Error;
use super::{
    contract::{PgRepositoryContract, GetPostsContract}, 
    data::{GetPostsAttributes, PaginatedPostsResponse}
};
use support::store::models::post::DisplayPost;

pub struct GetPost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetPostsContract for GetPost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_post(
        &self,
        post_id: &str
    ) -> Result<DisplayPost, Error> {
        let post = self
            .repository
            .get_post(post_id)
            .await?;

        Ok(
            post
        )
    }

    async fn get_users_posts_paginated(
        &self,
        user_id: &str,
        attibutes: GetPostsAttributes
    ) -> Result<PaginatedPostsResponse, Error> {
        self
            .repository
            .get_users_posts_paginated(user_id, attibutes)
            .await
            .map(PaginatedPostsResponse::from)
    }

    async fn get_feed_newest_posts_paginated(
        &self,
        attibutes: GetPostsAttributes
    ) -> Result<PaginatedPostsResponse, Error> {
        self
            .repository
            .get_feed_newest_posts_paginated(attibutes)
            .await
            .map(PaginatedPostsResponse::from)
    }

    async fn get_feed_best_reviewed_posts_paginated(
        &self,
        attibutes: GetPostsAttributes
    ) -> Result<PaginatedPostsResponse, Error> {
        self
            .repository
            .get_feed_best_reviewed_posts_paginated(attibutes)
            .await
            .map(PaginatedPostsResponse::from)
    }
}