use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods, JoinOnDsl, BoolExpressionMethods};
use error::Error;
use infrastructure::{
    db::Postgres, 
    schema::{comments, users, comment_likes}
};
use length_aware_paginator::{Response, Paginate};
use super::{
    contract::PgRepositoryContract, 
    data::{UserCommentsAttributes, CommentData}
};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository { 
    /// Fetches all posts comments
    async fn get_post_comments_paginated(
        &self,
        user_id: Option<String>,
        post_id: &str,
        attributes: UserCommentsAttributes
    ) -> Result<Response<CommentData>, Error> {
        let user_id = user_id.unwrap_or(String::from(""));
        let mut conn = self.pg_pool.connection()?;

        let mut query = 
            comments::table
                .left_join(users::table.on(users::id.eq(comments::user_id)))
                .left_join(comment_likes::table.on(
                    comments::id.eq(comment_likes::comment_id)
                    .and(comment_likes::user_id.eq(&user_id))
                ))
                .into_boxed();

        query = query
            .filter(comments::post_id.eq(post_id))
            .order(comments::created_at.desc());

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}