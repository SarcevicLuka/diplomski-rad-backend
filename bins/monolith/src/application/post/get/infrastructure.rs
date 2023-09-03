use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, JoinOnDsl, BoolExpressionMethods, select};
use diesel::dsl::{sum, count, exists};
use chrono::{Duration, Utc};
use error::Error;
use std::str;
use infrastructure::schema::post_likes;
use infrastructure::{
    db::Postgres, 
    schema::{posts, comments, watches, users}
};
use length_aware_paginator::{Response, Paginate};
use support::store::models::user::DisplayUser;
use support::store::models::watch_images::WatchImage;
use support::store::models::{
    post::Post, 
    watch::Watch,
    user::User
};
use super::data::{CombinedData, DisplayPostData, PostWithAvgScore, DisplayWatchImage};
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
        user_id: Option<String>,
        post_id: &str
    ) -> Result<DisplayPostData, Error> {
        let user_id = user_id.unwrap_or(String::from(""));
        let mut conn = self.pg_pool.connection()?;

        let post = Post::get_by_id(post_id, &mut conn)?;

        let watch = Watch::get_by_id(&post.watch_id, &mut conn)?;
        let watch_images = WatchImage::get_post_images(&watch.id, &mut conn)?;
        let mut display_watch_images: Vec<DisplayWatchImage> = vec![];

        for image in watch_images {
            display_watch_images.push(DisplayWatchImage::from(image));
        }

        let post_creator_id = Post::get_creator_id(post_id, &mut conn)?;
        let post_creator = User::get_by_id(&post_creator_id, &mut conn)?;

        let (sum_of_scores, num_of_comments)= comments::table
            .select((sum(comments::score), count(comments::id)))
            .filter(comments::post_id.eq(post_id))
            .get_result::<(Option<i64>, i64)>(&mut conn)?;

        let is_liked_by_user = 
            select(exists(
                post_likes::table.filter(post_likes::user_id.eq(user_id)
                .and(post_likes::post_id.eq(post_id)))
            ))
            .get_result::<bool>(&mut conn)?;

        let num_of_likes = post_likes::table
            .select(count(post_likes::id))
            .filter(post_likes::post_id.eq(post_id))
            .get_result::<i64>(&mut conn)?;

        let avg_non_rounded = match sum_of_scores {
            Some(avg) =>  {
                avg as f64 /num_of_comments as f64
            },
            None => 0.0,
        };
        let avg_rounded = (avg_non_rounded * 100.0).ceil() / 100.0;

        let post_with_avg_score = PostWithAvgScore {
            id: post.id,
            user_id: post.user_id,
            text: post.review,
            score: post.score,
            avg_comment_score: avg_rounded,
            num_of_likes,
            is_liked_by_user,
            created_at: post.created_at,
            updated_at: post.updated_at,
        };
        
        Ok(
            DisplayPostData {
                post: post_with_avg_score,
                creator: DisplayUser::from(post_creator),
                watch_data: watch,
                watch_images: display_watch_images
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
                .left_join(post_likes::table)
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
        user_id: Option<String>,
        attributes: GetPostsAttributes
    ) -> Result<Response<CombinedData>, Error> {
        let user_id = user_id.unwrap_or(String::from(""));
        let mut conn = self.pg_pool.connection()?;

        let now = Utc::now().naive_utc();
        let seven_days_ago = now - Duration::days(7);

        let mut query = 
            posts::table
                .left_join(users::table)
                .left_join(watches::table)
                .left_join(post_likes::table.on(
                    posts::id.eq(post_likes::post_id).and(post_likes::user_id.eq(&user_id))
                ))
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
        user_id: Option<String>,
        attributes: GetPostsAttributes
    ) -> Result<Response<CombinedData>, Error> {
        let user_id = user_id.unwrap_or(String::from(""));
        let mut conn = self.pg_pool.connection()?;

        let now = Utc::now().naive_utc();
        let seven_days_ago = now - Duration::days(7);

        let mut query = 
            posts::table
                .left_join(users::table)
                .left_join(watches::table)
                .left_join(post_likes::table.on(
                    posts::id.eq(post_likes::post_id).and(post_likes::user_id.eq(&user_id))
                ))
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