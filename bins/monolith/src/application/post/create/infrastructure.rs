use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::{post::{CreateNewPostData, Post}, watch::{CreateNewWatchData, Watch}, watch_images::{CreateNewWatchImageData, WatchImage}};
use super::contract::PgRepositoryContract;


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches pokemon by id
    async fn create_post(
        &self,
        post_data: CreateNewPostData
    ) -> Result<Post, Error> {
        let conn = self.pg_pool.connection()?;

        let post = Post::create(post_data, conn)?;

        Ok(
            post
        )
    } 

    async fn create_watch(
        &self,
        watch_data: CreateNewWatchData
    ) -> Result<String, Error> {
        let conn = self.pg_pool.connection()?;

        let watch = Watch::create(watch_data, conn)?;

        Ok(
            watch.id
        )
    }

    async fn create_watch_image(
        &self,
        watch_image_data: CreateNewWatchImageData
    ) -> Result<String, Error> {
        let conn = self.pg_pool.connection()?;

        let watch_image = WatchImage::create(watch_image_data, conn)?;

        Ok(
            watch_image.id
        )
    }
}