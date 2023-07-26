use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods, AsChangeset, Identifiable};
use infrastructure::{
    db::DbConnection,
    schema::{comments, comments::dsl::comments as edit_comment}
};
use error::Error;
use serde::{Serialize, Deserialize};

/// Struct for holding Comment data fron database
#[derive(Insertable, Queryable, Identifiable, AsChangeset, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = comments)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub post_id: String,
    pub text: String,
    pub score: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comment {
    /// Method for creating Comment
    pub fn create(data: CreateNewCommentData, mut connection: DbConnection) -> Result<Comment, Error> {
        diesel::insert_into(comments::table)
            .values(data)
            .get_result::<Comment>(&mut connection)
            .map_err(Error::from)
    }

    /// Helper method to find Comment by id
    pub fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<Comment, Error> {
        comments::table
            .filter(comments::id.eq(id))
            .get_result::<Comment>(connection)
            .map_err(Error::from)
    }

    /// Helper method to edit Comment by id
    pub fn edit_by_id(id: &str, text: &str, score: &i32, connection: &mut DbConnection) -> Result<Comment, Error> {
        diesel::update(edit_comment)
            .filter(comments::id.eq(id))
            .set((comments::text.eq(text), comments::score.eq(score)))
            .get_result::<Comment>(connection)
            .map_err(Error::from)
    }
}

impl From<Comment> for DisplayComment {
    fn from(value: Comment) -> Self {
        DisplayComment { 
            id: value.id,
            user_id: value.user_id,
            post_id: value.post_id,
            text: value.text,
            score: value.score,
            created_at: value.created_at,
            updated_at: value.updated_at
        }
    }
}

/// Struct for holding Comment data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = comments)]
#[serde(rename_all = "camelCase")]
pub struct DisplayComment {
    pub id: String,
    pub user_id: String,
    pub post_id: String,
    pub text: String,
    pub score: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Struct for creating Comment from Comment data
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = comments)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewCommentData {
    pub user_id: String,
    pub post_id: String,
    pub text: String,
    pub score: i32,
}

