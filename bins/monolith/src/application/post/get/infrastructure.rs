use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::dsl::{sum, count};
use chrono::{Duration, Utc};
use error::Error;
use infrastructure::{
    db::Postgres, 
    schema::{posts, comments, watches, users}
};
use length_aware_paginator::{Response, Paginate};
use support::store::models::user::{User, DisplayUser};
use support::store::models::{
    post::Post, 
    watch::Watch
};
use super::data::{DisplayPostWithAvgScore, CombinedData};
use super::{
    contract::PgRepositoryContract,
    data::GetPostsAttributes
};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches post by id
    async fn get_post(
        &self,
        post_id: &str
    ) -> Result<DisplayPostWithAvgScore, Error> {
        let mut conn = self.pg_pool.connection()?;

        let post = Post::get_by_id(post_id, &mut conn)?;
        let watch = Watch::get_by_id(&post.watch_id, &mut conn)?;
        let post_creator_id = Post::get_creator_id(post_id, &mut conn)?;
        let post_creator = User::get_by_id(&post_creator_id, &mut conn)?;

        let (sum_of_scores, num_of_comments)= comments::table
            .select((sum(comments::score), count(comments::id)))
            .filter(comments::post_id.eq(post_id))
            .get_result::<(Option<i64>, i64)>(&mut conn)?;

        let avg_non_rounded = sum_of_scores.unwrap() as f64 /num_of_comments as f64;
        let avg_rounded = (avg_non_rounded * 100.0).ceil() / 100.0;
        
        Ok(
            DisplayPostWithAvgScore {
                id: post.id,
                user_id: post.user_id, 
                watch_data: watch, 
                text: post.review, 
                score: post.score,
                avg_comment_score: avg_rounded,
                creator: DisplayUser::from(post_creator),
                created_at: post.created_at, 
                updated_at: post.updated_at 
            }
        )
    } 

    /// Fetches all users posts
    async fn get_users_posts_paginated(
        &self,
        user_id: &str,
        attributes: GetPostsAttributes
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

    /// Fetches newest posts in the last 7 days for the feed
    async fn get_feed_newest_posts_paginated(
        &self,
        attributes: GetPostsAttributes
    ) -> Result<Response<CombinedData>, Error> {
        let mut conn = self.pg_pool.connection()?;

        let now = Utc::now().naive_utc();
        let seven_days_ago = now - Duration::days(7);

        let mut query = 
            posts::table
                .left_join(users::table)
                .left_join(watches::table)
                .into_boxed();
                
        query = query
            .filter(posts::created_at.gt(seven_days_ago))
            .order(posts::created_at.desc());

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }

    /// Fetches best reviewed posts in the last 7 days for the feed
    async fn get_feed_best_reviewed_posts_paginated(
        &self,
        attributes: GetPostsAttributes
    ) -> Result<Response<CombinedData>, Error> {
        let mut conn = self.pg_pool.connection()?;

        let now = Utc::now().naive_utc();
        let seven_days_ago = now - Duration::days(7);

        let mut query = 
            posts::table
                .left_join(users::table)
                .left_join(watches::table)
                .into_boxed();
                
        query = query
            .filter(posts::created_at.gt(seven_days_ago))
            .order(posts::score.desc());

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}