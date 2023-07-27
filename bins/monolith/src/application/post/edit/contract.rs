use async_trait::async_trait;
use error::Error;
use support::store::models::{
    post::{Post, DisplayPost, EditPostData}, 
    watch::Watch
};

use super::data::{UserEditWatchData, UserEditPostData};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EditPostContract {
    async fn edit_post(&self, post_id: &str, post_data: UserEditPostData) -> Result<DisplayPost, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn edit_post(&self, post_id: &str, post_data: EditPostData) -> Result<Post, Error>;
    async fn edit_watch(&self, watch_id: &str, watch_data: UserEditWatchData) -> Result<Watch, Error>;
    async fn get_watch_id(&self, post_id: &str) -> Result<String, Error>;
}