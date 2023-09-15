use std::sync::Arc;
use actix_web::HttpRequest;
use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};
use error::Error;
use infrastructure::{
    db::{Postgres, get_postgres_connection_pool}, 
    schema::friendships
};
use support::{store::models::user::{User, DisplayUser}, helpers::jwt};
use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches user by id
    async fn get_user_by_id(
        &self,
        user_id: &str
    ) -> Result<DisplayUser, Error> {
        let mut conn = self.pg_pool.connection()?;

        let user = User::get_by_id(user_id, &mut conn)?;

        Ok(
            DisplayUser::from(user)
        )
    } 

    async fn am_following(
        &self,
        auth_user_id: &str,
        user_id: &str
    ) -> Result<bool, Error> {
        if auth_user_id == user_id {
            return Ok(false);
        }

        let mut conn = self.pg_pool.connection()?;

        let friendship_id = friendships::table
            .filter(
                friendships::user_requesting.eq(auth_user_id)
                .and(friendships::user_responding.eq(user_id))
            )
            .select(friendships::id)
            .get_result::<String>(&mut conn);

        match friendship_id  {
            Ok(id) => {
                if id.len() > 0 {
                    return Ok(true);
                }
                else {
                    return Ok(false);
                };
            },
            Err(_e) => return Ok(false)
        };
    }
}

fn try_extract_email_from_jwt(req: &HttpRequest) -> Result<String, Error> {
    let header = match req.headers().get("Authorization") {
        Some(head) => match head.to_str().ok() {
            Some(val) => val.to_string(),
            None => {String::from("")},
        },
        None => {String::from("")},
    };
    let mut split = header.split_whitespace();
    let auth_type = split.next();
    if Some("Bearer") == auth_type {
        if let Some(token) = split.next() {
            let secret = config::get_default("JWT_SECRET", "not_so_strong_secret");
            let claims = jwt::verify(token.to_string(), &secret)?;
            Ok(claims.email)
        } else {return Ok(String::from(""))}
    } else {return Ok(String::from(""))}
}

pub fn extract_valid_user(req: &HttpRequest) -> Result<Option<DisplayUser>, Error> {
    let pool = get_postgres_connection_pool();
    let postgres = Postgres {
        pool,
    };
    let user_email = try_extract_email_from_jwt(req)?;
    if user_email == "" {
        return Ok(None);
    }

    let user = User::get_by_email(&user_email, &mut postgres.connection()?);
    if user.is_err() {
        return Ok(None)
    }
    let authenticated_user = DisplayUser::from(user.unwrap());

    Ok(Some(authenticated_user))
}