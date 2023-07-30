use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;
use support::helpers::http::part_from_path;
use crate::application::user::get::follows::data::UserFollowsAttributes;
use super::super::contract::GetFollowsContract;

pub async fn handle_get_following<T: GetFollowsContract>(
    req: HttpRequest,
    attributes: web::Query<UserFollowsAttributes>,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let user_id = part_from_path::<String>(&req, "user_id")?;

    let attributes = attributes.into_inner().validate()?;

    let response = service
        .get_following_users(&user_id, attributes)
        .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}