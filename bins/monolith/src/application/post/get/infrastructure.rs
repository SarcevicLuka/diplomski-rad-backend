use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods};
use error::Error;
use infrastructure::{db::Postgres, schema::posts};
use length_aware_paginator::{Response, Paginate};
use support::store::models::{post::{Post, DisplayPost}, watch::Watch};
use super::{contract::PgRepositoryContract, data::UserPostsAttributes};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches post by id
    async fn get_post(
        &self,
        post_id: &str
    ) -> Result<DisplayPost, Error> {
        let mut conn = self.pg_pool.connection()?;

        let post = Post::get_by_id(post_id, &mut conn)?;
        let watch = Watch::get_by_id(&post.watch_id, &mut conn)?;

        Ok(
            DisplayPost {
                id: post.id,
                user_id: post.user_id, 
                watch_data: watch, 
                text: post.review, 
                created_at: post.created_at, 
                updated_at: post.updated_at 
            }
        )
    } 

    /// Fetches all users posts
    async fn get_users_posts_paginated(
        &self,
        user_id: &str,
        attributes: UserPostsAttributes
    ) -> Result<Response<Post>, Error> {
        let mut conn = self.pg_pool.connection()?;

        let query = 
            posts::table
                .into_boxed()
                .filter(posts::user_id.eq(user_id));

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}