use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use crate::application::user::get::info::infrastructure::extract_valid_user;

use super::super::contract::GetPostsContract;

pub async fn handle_get_post<T: GetPostsContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let user = extract_valid_user(&req)?;
    let post_id = part_from_path::<String>(&req, "post_id")?;

    let response = match user {
        Some(user) => {
                service
                    .get_post(Some(user.id), &post_id)
                    .await?
        },
        None => {
            service
                .get_post(None, &post_id)
                .await?
        }
    };

    Ok(HttpResponse::Ok()
        .json(response)
    )
}