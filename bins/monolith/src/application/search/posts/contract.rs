use async_trait::async_trait;
use error::Error;
use length_aware_paginator::Response;
use crate::application::post::get::data::{PaginatedPostsResponse, CombinedData};
use super::super::users::data::SearchAttributes;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetSearchedPosts {
    async fn search_posts(&self, attributes: SearchAttributes) -> Result<PaginatedPostsResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn search_posts(&self, attributes: SearchAttributes) -> Result<Response<CombinedData>, Error>;
}