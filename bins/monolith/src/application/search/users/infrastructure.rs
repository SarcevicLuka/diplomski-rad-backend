use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, ExpressionMethods, TextExpressionMethods, sql_function};
use error::Error;
use infrastructure::{
    db::Postgres, 
    schema::users
};
use length_aware_paginator::{Response, Paginate};
use support::store::models::user::User;
use super::{
    contract::PgRepositoryContract, 
    data::SearchAttributes
};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}


#[async_trait]
impl PgRepositoryContract for PgRepository { 
    /// Fetches all posts comments
    async fn search_users(
        &self,
        attributes: SearchAttributes
    ) -> Result<Response<User>, Error> {
        let mut conn = self.pg_pool.connection()?;
        
        let mut query = 
        users::table
        .into_boxed();
    
    if let Some(value) = &attributes.search_term {
            sql_function!(fn lower (a: diesel::sql_types::VarChar) -> diesel::sql_types::VarChar);
            let name = format!("%{}%", value.clone());
            query = query.filter(lower(users::first_name).like(name));
        }

        query = query
            .order(users::first_name.desc());

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}