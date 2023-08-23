use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods, prelude::Identifiable, Selectable};
use infrastructure::{
    db::DbConnection,
    schema::{posts, posts::dsl::posts as edit_post}
};
use error::Error;
use serde::{Serialize, Deserialize};

use super::watch::Watch;

/// Struct for holding post data fron database
#[derive(Insertable, Queryable, Serialize, Identifiable, Selectable, Deserialize, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = posts)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub watch_id: String,
    pub review: String,
    pub score: i32,
    pub num_of_likes: i32,
    pub num_of_comments: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Post {
    /// Method for creating post
    pub fn create(data: CreateNewPostData, mut connection: DbConnection) -> Result<Post, Error> {
        diesel::insert_into(posts::table)
            .values(data)
            .get_result::<Post>(&mut connection)
            .map_err(Error::from)
    }

    /// Method for deleting post
    pub fn delete(post_id: &str, mut connection: DbConnection) -> Result<usize, Error> {
        diesel::delete(posts::table)
            .filter(posts::id.eq(post_id))
            .execute(&mut connection)
            .map_err(Error::from)
    }

    /// Helper method to find post by id
    pub fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<Post, Error> {
        posts::table
            .filter(posts::id.eq(id))
            .get_result::<Post>(connection)
            .map_err(Error::from)
    }

    /// Helper method to find post by id
    pub fn get_watch_id(id: &str, connection: &mut DbConnection) -> Result<String, Error> {
        posts::table
            .filter(posts::id.eq(id))
            .select(posts::watch_id)
            .get_result(connection)
            .map_err(Error::from)
    }

    /// Helper method to find post creator id
    pub fn get_creator_id(id: &str, connection: &mut DbConnection) -> Result<String, Error> {
        posts::table
            .filter(posts::id.eq(id))
            .select(posts::user_id)
            .get_result(connection)
            .map_err(Error::from)
    }

    /// Helper method to edit post
    pub fn edit(id: &str, data: EditPostData, connection: &mut DbConnection) -> Result<Post, Error> {
        diesel::update(edit_post)
            .filter(posts::id.eq(id))
            .set((posts::review.eq(data.review), posts::score.eq(data.score)))
            .get_result::<Post>(connection)
            .map_err(Error::from)
    }
}

impl From<Post> for CreateNewPostData {
    fn from(value: Post) -> Self {
        CreateNewPostData { 
            user_id: value.user_id,
            watch_id: value.watch_id,
            review: value.review,
            score: value.score
        }
    }
}

/// Struct for holding post data to be displayed
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPost {
    pub id: String,
    pub user_id: String,
    pub watch_data: Watch,
    pub text: String,
    pub score: i32,
    pub num_of_likes: i32,
    pub num_of_comments: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Struct for creating Watch from post data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = posts)]
#[serde(rename_all = "camelCase")]
pub struct EditPostData {
    pub review: String,
    pub score: i32
}

/// Struct for creating Watch from post data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = posts)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewPostData {
    pub user_id: String,
    pub watch_id: String,
    pub review: String,
    pub score: i32
}

#[allow(dead_code)]
/// Method that will return created user with some given parameters
/// used as a helper when testing
pub fn testable(
    id: Option<&str>,
    user_id: Option<&str>,
    watch_id: Option<&str>,
    review: Option<&str>,
    score: Option<i32>,
) -> Post {
    Post {
        id: id.unwrap_or(&uuid::Uuid::new_v4().to_string()).to_string(),
        user_id: user_id.unwrap_or("test_user_id").to_string(),
        watch_id: watch_id.unwrap_or("watch_id").to_string(),
        review: review.unwrap_or("review").to_string(),
        score: score.unwrap_or(3),
        num_of_likes: 0,
        num_of_comments: 0,
        created_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    }
}