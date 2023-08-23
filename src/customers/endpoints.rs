use actix_web::{get, post, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, customers::customer::{create_customer, CreateCustomerSchema, get_customers, get_customer, delete_customer}};
#[derive( Deserialize, Serialize, Debug)]
pub struct NewCustomer {
    pub username: String,
    pub password: String
}

#[post("/customers")]
pub async fn create_customer_endpoint(
    body: web::Json<NewCustomer>,
    data: web::Data<AppState>
) -> impl Responder {
    let username = body.username.clone();
    let password = body.password.clone();

    let customer = create_customer(CreateCustomerSchema{username,password}, &data.db).await;

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

#[get("/customers")]
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

#[get("/customers/{user_id}")]
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

#[delete("/customers/{user_id}")]
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