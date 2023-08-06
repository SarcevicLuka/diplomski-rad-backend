use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use error::Error;
use infrastructure::{schema::friendships, db::DbConnection};
use serde::{Serialize, Deserialize};

/// Struct for holding post data fron database
#[derive(Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = friendships)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct UserFollow {
    pub id: String,
    pub user_requesting: String,
    pub user_responding: String,
    pub created_at: NaiveDateTime,
}

impl UserFollow {
    /// Method for creating user follow
    pub fn create(data: CreateNewUserFollowData, mut connection: DbConnection) -> Result<UserFollow, Error> {
        diesel::insert_into(friendships::table)
            .values(data)
            .get_result::<UserFollow>(&mut connection)
            .map_err(Error::from)
    }

    /// Method for deleting user follow
    pub fn delete(user_id: &str, unfollowed_user_id: &str, mut connection: DbConnection) -> Result<usize, Error> {
        diesel::delete(friendships::table)
            .filter(friendships::user_requesting.eq(user_id)
                .and(friendships::user_responding.eq(unfollowed_user_id)))
            .execute(&mut connection)
            .map_err(Error::from)
    }

    /// Method for getting ids of users user is following
    pub fn get_followed_user_ids(user_id: &str, mut connection: DbConnection) -> Result<Vec<String>, Error> {
        friendships::table
            .select(friendships::user_responding)
            .filter(friendships::user_requesting.eq(user_id))
            .order(friendships::created_at.desc())
            .get_results::<String>(&mut connection)
            .map_err(Error::from)
    }

     /// Method for getting ids of users user is following
     pub fn get_following_user_ids(user_id: &str, mut connection: DbConnection) -> Result<Vec<String>, Error> {
        friendships::table
            .select(friendships::user_requesting)
            .filter(friendships::user_responding.eq(user_id))
            .order(friendships::created_at.desc())
            .get_results::<String>(&mut connection)
            .map_err(Error::from)
    }
}

impl From<UserFollow> for CreateNewUserFollowData {
    fn from(value: UserFollow) -> Self {
        CreateNewUserFollowData { 
            user_requesting: value.user_requesting,
            user_responding: value.user_responding,
        }
    }
}

/// Struct for creating post_like from PostLikeData
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = friendships)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewUserFollowData {
    pub user_requesting: String,
    pub user_responding: String,
}

#[allow(dead_code)]
/// Method that will return created user with some given parameters
/// used as a helper when testing
pub fn testable(
    user_request_id: &str,
    user_response_id: &str,
) -> UserFollow {
    UserFollow { 
        id: uuid::Uuid::new_v4().to_string(), 
        user_requesting: user_request_id.to_string(), 
        user_responding: user_response_id.to_string(), 
        created_at: NaiveDateTime::parse_from_str("2023-04-19 08:00:00", "%Y-%m-%d %H:%M:%S")
        .unwrap() 
    }
}