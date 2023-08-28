use serde::Serialize;
use support::store::models::post_like::PostLike;

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserLikes {
    pub user_likes: Vec<PostLike>,
}