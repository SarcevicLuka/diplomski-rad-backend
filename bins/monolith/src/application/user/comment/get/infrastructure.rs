use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods};
use error::Error;
use infrastructure::{db::Postgres, schema::comments};
use length_aware_paginator::{Response, Paginate};
use support::store::models::comment::Comment;
use super::{contract::PgRepositoryContract, data::UserCommentsAttributes};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository { 
    /// Fetches all posts comments
    async fn get_post_comments_paginated(
        &self,
        post_id: &str,
        attributes: UserCommentsAttributes
    ) -> Result<Response<Comment>, Error> {
        debug!("{:#?}", attributes);
        let mut conn = self.pg_pool.connection()?;

        let query = 
            comments::table
                .into_boxed()
                .filter(comments::post_id.eq(post_id));

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}