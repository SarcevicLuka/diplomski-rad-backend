use async_trait::async_trait;
use error::Error;
use crate::application::post::get::data::PaginatedPostsResponse;

use super::{
    contract::{PgRepositoryContract, GetSearchedPosts}, 
    super::users::data::SearchAttributes
};

pub struct SearchPosts<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetSearchedPosts for SearchPosts<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn search_posts(
        &self,
        attibutes: SearchAttributes
    ) -> Result<PaginatedPostsResponse, Error> {
        self
            .repository
            .search_posts(attibutes)
            .await
            .map(PaginatedPostsResponse::from)
    }
}