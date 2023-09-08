use async_trait::async_trait;
use error::Error;
use super::{
    contract::{PgRepositoryContract, GetSearchedUsers}, 
    data::{PaginatedSearchResponse, SearchAttributes}
};

pub struct SearchUsers<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetSearchedUsers for SearchUsers<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn search_users(
        &self,
        attibutes: SearchAttributes
    ) -> Result<PaginatedSearchResponse, Error> {
        self
            .repository
            .search_users(attibutes)
            .await
            .map(PaginatedSearchResponse::from)
    }
}