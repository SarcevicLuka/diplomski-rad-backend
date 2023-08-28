use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use super::contract::GetUserLikesContract;

pub async fn handle_get_users_likes<T: GetUserLikesContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {    
    let user_id = part_from_path::<String>(&req, "user_id")?;

    let response = service
            .get_user_likes(&user_id)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}