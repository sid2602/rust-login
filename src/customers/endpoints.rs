use actix_web::{web, HttpResponse, Responder, Scope};
use uuid::Uuid;

use crate::{AppState, customers::customer::{create_customer, CreateCustomerSchema, get_customers, get_customer, delete_customer}, auth::middlewares::auth_middleware::RequireAuth};

use super::customer::CustomerRole;


pub fn customer_scope() -> Scope {
    web::scope("/customers")
        .route("",
            web::post().to(create_customer_endpoint).wrap(RequireAuth::allowed_roles(vec![
            CustomerRole::Admin, CustomerRole::Staff
        ])))
        .route("",
         web::get().to(get_customers_endpoint).wrap(RequireAuth::allowed_roles(vec![
            CustomerRole::Admin, CustomerRole::Staff
        ])))
        .route(
            "/{user_id}",
            web::get().to(get_customer_endpoint).wrap(RequireAuth::allowed_roles(vec![
                CustomerRole::Admin, CustomerRole::Staff
            ])),
        )
        .route(
            "",
            web::delete().to(delete_customer_endpoint).wrap(RequireAuth::allowed_roles(vec![
                CustomerRole::Admin, CustomerRole::Staff
            ])),
        )
}

pub async fn create_customer_endpoint(
    body: web::Json<CreateCustomerSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let firstname = body.firstname.clone();
    let lastname = body.lastname.clone();
    let email = body.email.clone();
    let password = body.password.clone();
    let role = body.role.clone();

    let customer = create_customer(CreateCustomerSchema{firstname,lastname,email,password,role}, &data.db).await;

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

pub async fn get_customers_endpoint(
    data: web::Data<AppState>
) -> impl Responder {
    let customers = get_customers(&data.db).await;

    match customers {
        Ok(customers) => {
            return HttpResponse::Ok().json(customers)
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

pub async fn get_customer_endpoint(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    let customer = get_customer(user_id,&data.db).await;

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

pub async fn delete_customer_endpoint(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    let delete_result = delete_customer(user_id,&data.db).await;

    match delete_result {
        Ok(result) => {
            return HttpResponse::Ok().json(serde_json::json!({"message": result}));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}