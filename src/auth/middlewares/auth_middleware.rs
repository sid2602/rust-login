use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{http, web, HttpMessage};
use futures_util::FutureExt;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{DecodingKey, decode, Validation};
use uuid::Uuid;
use std::rc::Rc;
use std::task::Poll;
use crate::AppState;
use crate::auth::jwt::TokenClaims;
use crate::config::error_response::{ErrorResponse, ErrorStatus};
use crate::customers::customer::{CustomerRole, get_customer, Customer};

pub struct RequireAuth {
    pub allowed_roles: Rc<Vec<CustomerRole>>
}

impl RequireAuth {
    pub fn allowed_roles(allowed_roles: Vec<CustomerRole> ) -> Self {
        RequireAuth { allowed_roles: Rc::new(allowed_roles) }
    }
}

impl <S> Transform<S,ServiceRequest> for RequireAuth
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Rc<Vec<CustomerRole>>
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req.cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: ErrorStatus::Unauthorized,
                message: "You are not logged in, please provide token".to_string(),
            };
            return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: ErrorStatus::Unauthorized,
                    message: "Invalid token".to_string(),
                };
                return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
            }
        };


        let cloned_app_state = data.clone();
        let allowed_roles = self.allowed_roles.clone();
        let srv = Rc::clone(&self.service);

        async move {
            let user_id = Uuid::parse_str(claims.sub.as_str()).unwrap();
            let customer =  match get_customer(user_id,&cloned_app_state.db).await {
                Ok(customer) => customer,
                Err(_) => {
                    return Err(ErrorUnauthorized(ErrorResponse {
                        status: ErrorStatus::Unauthorized,
                        message: "User not exists".to_string()
                    }))
                }
            };

            if allowed_roles.contains(&customer.role) {

                req.extensions_mut().insert::<Customer>(customer);
                let res = srv.call(req).await?;
                Ok(res)

            } else {

                Err(ErrorUnauthorized(ErrorResponse {
                    status: ErrorStatus::BadRequest,
                    message: "failed with jwt check".to_string()
                }))
            }

        }.boxed_local()

    }
}