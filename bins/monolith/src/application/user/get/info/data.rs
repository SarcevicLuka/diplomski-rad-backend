use serde::Serialize;
use support::store::models::user::DisplayUser;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResponse {
    pub user_data: DisplayUser,
    pub am_following: bool
}