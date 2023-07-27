use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::{
    post::{Post, EditPostData}, 
    watch::Watch
};
use super::{
    contract::PgRepositoryContract, 
    data::UserEditWatchData
};


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Edits post by id
    async fn edit_post(
        &self,
        post_id: &str,
        post_data: EditPostData
    ) -> Result<Post, Error> {
        let mut conn = self.pg_pool.connection()?;

        let post = Post::edit(post_id, post_data, &mut conn)?;

        Ok(
            post
        )
    } 

    async fn edit_watch(
        &self,
        watch_id: &str,
        watch_data: UserEditWatchData
    ) -> Result<Watch, Error> {
        let mut conn = self.pg_pool.connection()?;

        let watch = Watch::edit(watch_id, watch_data.insertable(), &mut conn)?;

        Ok(
            watch
        )
    }

    async fn get_watch_id(
        &self,
        post_id: &str,
    ) -> Result<String, Error> {
        let mut conn = self.pg_pool.connection()?;

        let watch_id = Post::get_watch_id(post_id, &mut conn)?;

        Ok(
            watch_id
        )
    }
}