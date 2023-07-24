use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::post::Post;
use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches post by id
    async fn get_post(
        &self,
        post_id: &str
    ) -> Result<Post, Error> {
        let mut conn = self.pg_pool.connection()?;

        let post = Post::get_by_id(post_id, &mut conn)?;

        Ok(
            post
        )
    } 

    //async fn get_all_posts(
    //    &self,
    //    user_id: &str
    //) -> Result<Post, Error> {
    //    let conn = self.pg_pool.connection()?;
//
    //    let watch = Watch::create(watch_data, conn)?;
//
    //    Ok(
    //        watch.id
    //    )
    //}
}