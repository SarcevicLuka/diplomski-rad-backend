use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::{
    store::models::user::DisplayUser, 
    helpers::http::part_from_path
};
use super::contract::GetUserContract;

pub async fn handle_get_user<T: GetUserContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let Some(_user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("not authorized".to_string()));
    };

    let user_id = part_from_path::<String>(&req, "user_id")?;

    let response = 
        service
            .get_user_by_id(&user_id)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}