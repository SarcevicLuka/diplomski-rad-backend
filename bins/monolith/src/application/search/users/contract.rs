use async_trait::async_trait;
use error::Error;
use length_aware_paginator::Response;
use support::store::models::user::User;
use super::data::{SearchAttributes, PaginatedSearchResponse};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetSearchedUsers {
    async fn search_users(&self, attributes: SearchAttributes) -> Result<PaginatedSearchResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn search_users(&self, attributes: SearchAttributes) -> Result<Response<User>, Error>;
}