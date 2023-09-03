use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;
use crate::application::user::get::info::infrastructure::extract_valid_user;

use super::super::data::GetPostsAttributes;
use super::super::contract::GetPostsContract;
pub async fn handle_get_newest_posts<T: GetPostsContract>(
    req: HttpRequest,
    service: web::Data<T>,
    attributes: web::Query<GetPostsAttributes>,
) -> Result<HttpResponse, Error> {
    let user = extract_valid_user(&req)?;
    let attributes = attributes.into_inner().validate()?;

    let response = match user {
        Some(user) => {
                service
                    .get_feed_newest_posts_paginated(Some(user.id), attributes)
                    .await?
        },
        None => {
            service
                .get_feed_newest_posts_paginated(None, attributes)
                .await?
        }
    };


    //let response = 
    //    service
    //        .get_feed_newest_posts_paginated(None, attributes)
    //        .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}