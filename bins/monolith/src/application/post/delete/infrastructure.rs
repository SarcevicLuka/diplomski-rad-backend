use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use super::contract::PgRepositoryContract;
use support::store::models::post::Post;


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn delete_post(
        &self,
        post_id: &str
    ) -> Result<(), Error> {
        let conn = self.pg_pool.connection()?;

        let num_of_deleted_rows = Post::delete(post_id, conn)?;
        if num_of_deleted_rows == 0 {
            return Err(Error::Request("User or post id is invalid".to_string()));
        }

        Ok(())
    } 
}