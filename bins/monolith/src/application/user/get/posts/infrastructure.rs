use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods};
use error::Error;
use infrastructure::{
    db::Postgres, 
    schema::{users, posts, watches}
};
use length_aware_paginator::{Response, Paginate};
use crate::application::post::get::data::CombinedData;
use super::{
    contract::PgRepositoryContract, 
    data::UserPostsAttributes
};

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
    ) -> Result<Response<CombinedData>, Error> {
        let mut conn = self.pg_pool.connection()?;

        let mut query = 
            posts::table
                .left_join(users::table)
                .left_join(watches::table)
                .into_boxed();

        query = query
            .filter(posts::user_id.eq(user_id))
            .order_by(posts::created_at.desc());

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}