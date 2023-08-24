use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::{helpers::http::part_from_path, store::models::user::DisplayUser};
use super::contract::GetUserContract;

pub async fn handle_get_user<T: GetUserContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let user = req.extensions_mut().remove::<DisplayUser>();
    let user_id = part_from_path::<String>(&req, "user_id")?;

    let response = match user {
        Some(user) => {
            service
                .get_user_by_id(Some(user.id), &user_id)
                .await?
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