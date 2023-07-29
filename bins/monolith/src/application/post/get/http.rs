use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;
use super::contract::GetPostsContract;

pub async fn handle_get_post<T: GetPostsContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let post_id = part_from_path::<String>(&req, "post_id")?;

    let response = 
        service
            .get_post(&post_id)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}