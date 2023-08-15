use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use validr::Validation;
use support::store::models::user::DisplayUser;
use super::{contract::CreatePostContract, data::UserPostData};

pub async fn handle_create_post<T: CreatePostContract>(
    req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<UserPostData>
) -> Result<HttpResponse, Error> {
    let Some(user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("Not authorized".to_string()));
    };

    let data = data.into_inner().validate()?;

    let response = 
        service
            .create_post(&user.id, data)
            .await?;

    Ok(HttpResponse::Created()
        .json(response)
    )
}