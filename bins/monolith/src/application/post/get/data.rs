use length_aware_paginator::Response;
use serde::{Deserialize, Serialize};
use support::store::models::post::Post;
use validr::*;

/// Struct that holds users paginated pokedex
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedUsersPostsResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub data: Vec<Post>,
}

impl From<Response<Post>> for PaginatedUsersPostsResponse {
    fn from(source: Response<Post>) -> Self {
        Self {
            page: source.page,
            per_page: source.per_page,
            total: source.total,
            last_page: source.last_page,
            data: source.data,
        }
    }
}

/// Struct that holds users pagination attributes
#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPostsAttributes {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
    pub name: Option<String>,
}

impl Validation for UserPostsAttributes {
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