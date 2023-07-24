use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods};
use infrastructure::{
    db::DbConnection,
    schema::posts
};
use error::Error;
use serde::{Serialize, Deserialize};

/// Struct for holding post data fron database
#[derive(Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = posts)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub watch_id: String,
    pub review: String,
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

    /// Helper method to find post by id
    pub fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<Post, Error> {
        posts::table
            .filter(posts::id.eq(id))
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
        }
    }
}

/// Struct for creating Watch from post data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = posts)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewPostData {
    pub user_id: String,
    pub watch_id: String,
    pub review: String,
}