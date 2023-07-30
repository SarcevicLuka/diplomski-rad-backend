use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::{
    store::models::user::DisplayUser, 
    helpers::http::part_from_path
};
use super::super::contract::FollowUserContract;

pub async fn handle_follow_user<T: FollowUserContract>(
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let Some(request_user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("not authorized".to_string()));
    };

    let response_user_id = part_from_path::<String>(&req, "user_id")?;

    let response = 
        service
            .follow_user(&request_user.id, &response_user_id)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}