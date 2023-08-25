use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use crate::application::user::get::info::infrastructure::extract_valid_user;

use super::contract::GetUserContract;

pub async fn handle_get_user<T: GetUserContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let user = extract_valid_user(&req)?;
    let user_id = part_from_path::<String>(&req, "user_id")?;

    let response = match user {
        Some(user) => {
            if user.id == user_id {
                service
                    .get_user_by_id(None, &user_id)
                    .await?
            } else {
                service
                    .get_user_by_id(Some(user.id), &user_id)
                    .await?
            }
        },
        None => {
            service
                .get_user_by_id(None, &user_id)
                .await?
        }
    };

    Ok(HttpResponse::Ok()
        .json(response)
    )
}