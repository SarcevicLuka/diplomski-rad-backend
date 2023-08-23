// use actix_web::{HttpRequest, web, HttpResponse};
// use error::Error;
// use validr::Validation;
// use super::super::data::GetPostsAttributes;
// use super::super::contract::GetPostsContract;

// pub async fn handle_get_newest_posts<T: GetPostsContract>(
//     _req: HttpRequest,
//     service: web::Data<T>,
//     attributes: web::Query<GetPostsAttributes>,
// ) -> Result<HttpResponse, Error> {
//     let attributes = attributes.into_inner().validate()?;

//     let response = 
//         service
//             .get_feed_newest_posts_paginated(attributes)
//             .await?;

//     Ok(HttpResponse::Ok()
//         .json(response)
//     )
// }