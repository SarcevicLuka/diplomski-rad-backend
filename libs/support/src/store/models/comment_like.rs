use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};
use error::Error;
use infrastructure::{schema::comment_likes, db::DbConnection};
use serde::{Serialize, Deserialize};

/// Struct for holding post data fron database
#[derive(Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = comment_likes)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct CommentLike {
    pub id: String,
    pub user_id: String,
    pub comment_id: String,
    pub created_at: NaiveDateTime,
}

impl CommentLike {
    /// Method for creating like
    pub fn create(data: CreateNewCommentLikeData, mut connection: DbConnection) -> Result<CommentLike, Error> {
        diesel::insert_into(comment_likes::table)
            .values(data)
            .get_result::<CommentLike>(&mut connection)
            .map_err(Error::from)
    }

    /// Method for deleting like
    pub fn delete(data: CreateNewCommentLikeData, mut connection: DbConnection) -> Result<usize, Error> {
        diesel::delete(comment_likes::table)
            .filter(comment_likes::user_id.eq(data.user_id)
                .and(comment_likes::comment_id.eq(data.comment_id)))
            .execute(&mut connection)
            .map_err(Error::from)
    }
}

impl From<CommentLike> for CreateNewCommentLikeData {
    fn from(value: CommentLike) -> Self {
        CreateNewCommentLikeData { 
            user_id: value.user_id,
            comment_id: value.comment_id,
        }
    }
}

/// Struct for creating post_like from CommentLikeData
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = comment_likes)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewCommentLikeData {
    pub user_id: String,
    pub comment_id: String,
}