use error::Error;
use actix_web::{web, HttpRequest, HttpResponse};
use validr::Validation;
use super::{data::RegistrationUserData, contract::RegistrationContract};

pub async fn handle_register<T: RegistrationContract>(
    _req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<RegistrationUserData>
) -> Result<HttpResponse, Error> {
    let data = data.into_inner().validate()?;
    let (authenticated_user, token) = service.register(data).await?;
    
    Ok(HttpResponse::Created()
        .insert_header(("Authorization", token))
        .json(authenticated_user)
    )
}