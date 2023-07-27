use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use validr::Validation;
use super::{contract::GetCommentsContract, data::UserCommentsAttributes};

pub async fn handle_get_post_comments<T: GetCommentsContract>(
    req: HttpRequest,
    attributes: web::Query<UserCommentsAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let post_id = part_from_path::<String>(&req, "post_id")?;

    let attributes = attributes.into_inner().validate()?;

    let response = 
        service
            .get_post_comments_paginated(&post_id, attributes)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}