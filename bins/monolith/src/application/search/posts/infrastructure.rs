use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods, sql_function, JoinOnDsl, BoolExpressionMethods};
use error::Error;
use infrastructure::{
    db::Postgres, 
    schema::{users, posts, watches, post_likes}
};
use length_aware_paginator::{Response, Paginate};
use crate::application::post::get::data::CombinedData;

use super::{
    contract::PgRepositoryContract, 
    super::users::data::SearchAttributes
};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}


#[async_trait]
impl PgRepositoryContract for PgRepository { 
    /// Fetches all posts comments
    async fn search_posts(
        &self,
        attributes: SearchAttributes
    ) -> Result<Response<CombinedData>, Error> {
        let mut conn = self.pg_pool.connection()?;
        
        let mut query = 
            posts::table
                .left_join(users::table)
                .left_join(watches::table)
                .left_join(post_likes::table.on(
                    posts::id
                        .eq
                        (post_likes::post_id)
                        .and
                        (post_likes::user_id.eq("b411fb48-5fe1-41a3-a1be-3e690b4c69d0"))
                ))
                .into_boxed();
    
        if let Some(value) = &attributes.search_term {
            sql_function!(fn lower (a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);
            let name = format!("%{}%", value.clone());
            query = query.filter(lower(watches::brand).like(name));
        }

        query = query
            .order(posts::created_at.desc());

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}