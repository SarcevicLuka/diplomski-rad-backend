use actix_web::{HttpRequest, HttpResponse, web};
use error::Error;
use validr::Validation;
use super::{
    data::LoginUserData, 
    contract::LoginContract
};

pub async fn handle_login<T: LoginContract>(
    _req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<LoginUserData>
) -> Result<HttpResponse, Error> {
    let data = data.into_inner().validate()?;
    let (user, token) = service.login(data).await?;
    
    Ok(HttpResponse::Ok()
        .insert_header(("Authorization", token))
        .json(user)
    )
}