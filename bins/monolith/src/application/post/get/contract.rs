use async_trait::async_trait;
use error::Error;
use support::store::models::{
    post::Post, 
    user::User, 
    watch::Watch
};
use length_aware_paginator::Response;
use super::data::{GetPostsAttributes, PaginatedPostsResponse, DisplayPostWithAvgScore, CombinedData};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetPostsContract {
    async fn get_post(&self, post_id: &str) -> Result<DisplayPostWithAvgScore, Error>;
    //async fn get_users_posts_paginated(&self, user_id: &str, attibutes: GetPostsAttributes) -> Result<PaginatedPostsResponse, Error>;
    //async fn get_feed_newest_posts_paginated(&self, attibutes: GetPostsAttributes) -> Result<PaginatedPostsResponse, Error>;
    //async fn get_feed_best_reviewed_posts_paginated(&self, attibutes: GetPostsAttributes) -> Result<PaginatedPostsResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_post(&self, post_id: &str) -> Result<DisplayPostWithAvgScore, Error>;
    //async fn get_users_posts_paginated(&self, user_id: &str, attibutes: GetPostsAttributes) -> Result<Response<(Option<Post>, Option<User>, Option<Watch>)>, Error>;
    //async fn get_feed_newest_posts_paginated(&self, attibutes: GetPostsAttributes) -> Result<Response<CombinedData>, Error>;
    //async fn get_feed_best_reviewed_posts_paginated(&self, attibutes: GetPostsAttributes) -> Result<Response<CombinedData>, Error>;
    //async fn get_users_posts_paginated(&self, user_id: &str, attibutes: GetPostsAttributes) -> Result<Response<CombinedData>, Error>;
    //async fn get_feed_newest_posts_paginated(&self, attibutes: GetPostsAttributes) -> Result<Response<CombinedData>, Error>;
    //async fn get_feed_best_reviewed_posts_paginated(&self, attibutes: GetPostsAttributes) -> Result<Response<CombinedData>, Error>;
}