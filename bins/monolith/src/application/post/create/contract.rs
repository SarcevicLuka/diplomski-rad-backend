use async_trait::async_trait;
use error::Error;
use support::store::models::{post::{CreateNewPostData, Post}, watch::CreateNewWatchData};

use super::data::UserPostData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CreatePostContract {
    async fn create_post(&self, user_id: &str, post_data: UserPostData) -> Result<Post, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn create_post(&self, post_data: CreateNewPostData) -> Result<Post, Error>;
    async fn create_watch(&self, watch_data: CreateNewWatchData) -> Result<String, Error>;
}