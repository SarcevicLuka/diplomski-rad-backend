use diesel::prelude::Queryable;
use length_aware_paginator::Response;
use serde::{Deserialize, Serialize};
use support::store::models::{comment::Comment, user::{User, DisplayUser}, post_like::PostLike};
use validr::*;

/// Struct that holds users paginated pokedex
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedPostsCommentsResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub data: Vec<DisplayCommentData>,
}

impl From<Response<CommentData>> for PaginatedPostsCommentsResponse {
    fn from(source: Response<CommentData>) -> Self {
        Self {
            page: source.page,
            per_page: source.per_page,
            total: source.total,
            last_page: source.last_page,
            data: source
                .data
                .into_iter()
                .map(DisplayCommentData::from)
                .collect::<Vec<DisplayCommentData>>(),
        }
    }
}

/// Struct that holds users paginated pokedex
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayCommentData {
    pub comment: Comment,
    pub creator: DisplayUser,
    pub is_liked: bool,
}

impl From<CommentData> for DisplayCommentData {
    fn from(source: CommentData) -> Self {
        DisplayCommentData { 
            comment: source.comment, 
            creator: DisplayUser::from(source.creator.unwrap()),
            is_liked: match source.like {
                Some(_like) => true,
                None => false,
            }
        }
    }
}

/// Struct that holds users paginated pokedex
#[derive(Clone, Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct CommentData {
    comment: Comment,
    creator: Option<User>,
    like: Option<PostLike>
}

/// Struct that holds users pagination attributes
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserCommentsAttributes {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
    pub name: Option<String>,
}

impl Validation for UserCommentsAttributes {
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