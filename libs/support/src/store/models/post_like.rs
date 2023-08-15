use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};
use error::Error;
use infrastructure::{schema::post_likes, db::DbConnection};
use serde::{Serialize, Deserialize};

/// Struct for holding post data fron database
#[derive(Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = post_likes)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct PostLike {
    pub id: String,
    pub user_id: String,
    pub post_id: String,
    pub created_at: NaiveDateTime,
}

impl PostLike {
    /// Method for creating like
    pub fn create(data: CreateNewPostLikeData, mut connection: DbConnection) -> Result<PostLike, Error> {
        diesel::insert_into(post_likes::table)
            .values(data)
            .get_result::<PostLike>(&mut connection)
            .map_err(Error::from)
    }

    /// Method for deleting like
    pub fn delete(data: CreateNewPostLikeData, mut connection: DbConnection) -> Result<usize, Error> {
        diesel::delete(post_likes::table)
            .filter(post_likes::user_id.eq(data.user_id)
                .and(post_likes::post_id.eq(data.post_id)))
            .execute(&mut connection)
            .map_err(Error::from)
    }
}

impl From<PostLike> for CreateNewPostLikeData {
    fn from(value: PostLike) -> Self {
        CreateNewPostLikeData { 
            user_id: value.user_id,
            post_id: value.post_id,
        }
    }
}

/// Struct for creating post_like from PostLikeData
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = post_likes)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewPostLikeData {
    pub user_id: String,
    pub post_id: String,
}

#[allow(dead_code)]
/// Method that will return created user with some given parameters
/// used as a helper when testing
pub fn testable(
    id: Option<&str>,
    user_id: Option<&str>,
    post_id: Option<&str>,
) -> PostLike {
    PostLike { 
        id: id.unwrap_or(&uuid::Uuid::new_v4().to_string()).to_string(),
        user_id: user_id.unwrap_or("test_user_id").to_string(), 
        post_id: post_id.unwrap_or("test_post_id").to_string(), 
        created_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
        .unwrap() 
    }
}