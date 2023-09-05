use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use validr::Validation;
use crate::application::user::get::info::infrastructure::extract_valid_user;

use super::{contract::GetCommentsContract, data::UserCommentsAttributes};

pub async fn handle_get_post_comments<T: GetCommentsContract>(
    req: HttpRequest,
    attributes: web::Query<UserCommentsAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let user = extract_valid_user(&req)?;
    let post_id = part_from_path::<String>(&req, "post_id")?;

    let attributes = attributes.into_inner().validate()?;

    let response = match user {
        Some(user) => {
                service
                    .get_post_comments_paginated(Some(user.id), &post_id, attributes)
                    .await?
        },
        None => {
            service
                .get_post_comments_paginated(None, &post_id, attributes)
                .await?
        }
    };

    //let response = 
    //    service
    //        .get_post_comments_paginated(&post_id, attributes)
    //        .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}