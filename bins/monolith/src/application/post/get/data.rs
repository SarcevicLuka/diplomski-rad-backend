use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use length_aware_paginator::Response;
use serde::{Deserialize, Serialize};
use support::store::models::{
    post::Post, watch::Watch, 
    user::{User, DisplayUser}, 
    watch_images::WatchImage, post_like::PostLike
};
use validr::*;
use std::str;

/// Struct that holds users paginated pokedex
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedPostsResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub data: Vec<PostListResponse>,
}

impl From<Response<CombinedData>> for PaginatedPostsResponse {
    fn from(source: Response<CombinedData>) -> Self {
        Self {
            page: source.page,
            per_page: source.per_page,
            total: source.total,
            last_page: source.last_page,
            data: source
                .data
                .into_iter()
                .map(PostListResponse::from)
                .collect::<Vec<PostListResponse>>(),
        }
    }
}

/// Struct that holds posts for the frontend
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostListResponse {
    pub post: Post,
    pub creator: DisplayUser,
    pub watch_data: Watch,
    pub is_liked: bool,
}

impl From<CombinedData> for PostListResponse {
    fn from(source: CombinedData) -> Self {
        PostListResponse { 
            post: source.post,
            creator: DisplayUser::from(source.user.unwrap()),
            watch_data: source.watch.unwrap(),
            is_liked: match source.like {
                Some(_post_like) => true,
                None => false,
            }
        }
    }
}

/// Struct that holds users pagination attributes
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPostsAttributes {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
    pub name: Option<String>,
}

impl Validation for GetPostsAttributes {
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_uppercase!(sort),
            modifier_lowercase!(name),
            Modifier::new("sort", |obj: &mut Self| {
                if obj.sort.is_none() {
                    obj.sort = Some("DESC".to_string())
                }
            }),
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_in!(sort, vec!["ASC".to_string(), "DESC".to_string()]),
        ]
    }
}

/// Struct for holding post data with average comments score
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostWithAvgScore {
    pub id: String,
    pub user_id: String,
    pub text: String,
    pub score: i32,
    pub avg_comment_score: f64,
    pub num_of_likes: i64,
    pub is_liked_by_user: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Struct for holding post data to be displayed
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPostData {
    pub post: PostWithAvgScore,
    pub creator: DisplayUser,
    pub watch_data: Watch,
    pub watch_images: Vec<DisplayWatchImage>
}

#[derive(Queryable, PartialEq, Debug)]
pub struct CombinedData {
    post: Post,
    user: Option<User>,
    watch: Option<Watch>,
    like: Option<PostLike>
}

#[derive(Queryable, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayWatchImage {
    pub id: String,
    pub watch_id: String,
    pub data: String,
}

impl From<WatchImage> for DisplayWatchImage {
    fn from(value: WatchImage) -> Self {
        Self { 
            id: value.id, 
            watch_id: value.watch_id,
            data: str::from_utf8(&value.data).unwrap().to_string()
        }
    }
}