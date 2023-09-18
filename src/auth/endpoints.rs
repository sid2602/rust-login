use actix_web::{post, get, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use serde::{Deserialize, Serialize};

use crate::{AppState, customers::customer::{create_customer, CreateCustomerSchema, get_customer_by_email, CustomerRole, Customer,}, auth::{auth::{check_password_is_valid_when_register, hash_password, is_password_valid_with_hashed_password}, jwt::{create_token, set_token_in_cookies, remove_token_from_cookies}}};

#[derive( Deserialize, Serialize, Debug)]
pub struct RegisterCustomerSchema {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub repeated_password: String,
}

#[post("/auth/register")]
pub async fn register_customer_endpoint(
    body: web::Json<RegisterCustomerSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let email = body.email.clone();
    let firstname = body.firstname.clone();
    let lastname = body.lastname.clone();
    let password = body.password.clone();
    let repeated_password = body.repeated_password.clone();

    let valid_password = match check_password_is_valid_when_register(password,repeated_password) {
        Ok(password) => {
           password
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let hashed_password = match hash_password(valid_password) {
        Ok(hashed_password) => {
            hashed_password
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let customer = create_customer(CreateCustomerSchema{firstname,lastname, email, role: Some(CustomerRole::Viewer) ,password: hashed_password}, &data.db).await;

    match customer {
        Ok(customer) => {
            return HttpResponse::Ok().json(customer)
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[derive( Deserialize, Serialize, Debug)]
pub struct LoginCustomerSchema {
    pub email: String,
    pub password: String,
}

#[post("/auth/login")]
pub async fn login_customer_endpoint(
    body: web::Json<LoginCustomerSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let email = body.email.clone();
    let password = body.password.clone();

    let customer = match get_customer_by_email(email, &data.db).await {
        Ok(customer) => {
           customer
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let is_password_correct = is_password_valid_with_hashed_password(password,customer.clone().password);

    let token = create_token(customer.id, &data.env.jwt_secret);

    let cookie = set_token_in_cookies(token.clone());

    if is_password_correct {
        return HttpResponse::Ok().cookie(cookie).json(serde_json::json!({"jwt": token}))
    }

    HttpResponse::InternalServerError()
    .json(serde_json::json!({"status": "error","message": format!("{:?}", "Wrong username or password")}))
}

#[get("/auth/logout")]
pub async fn logout_customer_endpoint() -> impl Responder {

    let cookie = remove_token_from_cookies();

    HttpResponse::InternalServerError()
    .cookie(cookie)
    .json(serde_json::json!({"status": "success"}))
}

#[get("/me")]
pub async fn get_me_endpoint(
    req: HttpRequest,
) -> impl Responder {

    match req.extensions().get::<Customer>() {
        Some(customer) => {
            return HttpResponse::Ok().json(customer)
        },
        None => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": "Invalid customer with this jwt"}));
        }
    }
}