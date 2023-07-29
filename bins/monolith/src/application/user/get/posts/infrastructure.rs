use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods};
use diesel::dsl::sql;
use error::Error;
use infrastructure::{db::Postgres, schema::posts};
use length_aware_paginator::{Response, Paginate};
use support::store::models::post::Post;
use super::{contract::PgRepositoryContract, data::UserPostsAttributes};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches all users posts
    async fn get_users_posts_paginated(
        &self,
        user_id: &str,
        attributes: UserPostsAttributes
    ) -> Result<Response<(Post, i64)>, Error> {
        use infrastructure::schema::post_likes::dsl::post_likes as post_likes_count;

        let mut conn = self.pg_pool.connection()?;

        let mut query = 
            posts::table
                .left_join(post_likes_count)
                .group_by(posts::id)
                .select((posts::all_columns, sql::<diesel::sql_types::BigInt>("COUNT(post_likes.id) as num_likes")))
                .into_boxed();
                
        query = query.filter(posts::user_id.eq(user_id));

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}