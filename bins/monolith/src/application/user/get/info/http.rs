use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::store::models::user::DisplayUser;
use super::contract::GetUserContract;

pub async fn handle_get_user<T: GetUserContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let Some(user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("not authorized".to_string()));
    };

    let response = 
        service
            .get_user_by_id(&user.id)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}