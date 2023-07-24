use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;
use support::helpers::http::part_from_path;
use crate::application::user::post::get::data::UserPostsAttributes;

use super::super::contract::GetPostsContract;

pub async fn handle_users_posts<T: GetPostsContract>(
    req: HttpRequest,
    attributes: web::Query<UserPostsAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let user_id = part_from_path::<String>(&req, "user_id")?;

    let attributes = attributes.into_inner().validate()?;

    let response = 
        service
            .get_users_posts_paginated(&user_id, attributes)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}