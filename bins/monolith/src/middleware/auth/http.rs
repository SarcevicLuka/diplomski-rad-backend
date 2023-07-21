use super::{
    domain::Session, 
    infrastructure::{Request, PgRepository}, 
    contract::SessionContract
};
use futures::future::LocalBoxFuture;
use futures_util::future::{ok, Ready};
use infrastructure::state::AppState;
use actix_web::{
    dev::{
        Transform, 
        ServiceRequest, 
        Service, 
        ServiceResponse, 
        forward_ready
    }, 
    body::BoxBody, Error, ResponseError, HttpMessage
};

pub struct AuthLogin;

impl<S> Transform<S, ServiceRequest> for AuthLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let state = req.app_data::<AppState>().unwrap();

        let mut service = Session {
            request: Request { req: &req },
            repository: PgRepository {
                pg_connection: state.postgres_connection(),
            },
        };

        match service.extract_valid_user() {
            Ok(authenticated_user) => {
                req.extensions_mut()
                    .insert(authenticated_user);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            },
            Err(e) => {
                Box::pin(
                    async move {Ok(
                        ServiceResponse::new(req.into_parts().0, e.error_response())
                    )},
                )
            }
        }
    }
}