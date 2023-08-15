use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::helpers::http::part_from_path;
use support::store::models::user::DisplayUser;
use super::contract::DeletePostContract;

pub async fn handle_delete_post<T: DeletePostContract>(
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let Some(_user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("Not authorized".to_string()));
    };

    let post_id = part_from_path::<String>(&req, "post_id")?;

    service
        .delete_post(&post_id)
        .await?;

    Ok(HttpResponse::Ok().finish())
}