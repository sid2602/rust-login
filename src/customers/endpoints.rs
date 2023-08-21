use actix_web::{get, post, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::customer::Customer;

use crate::AppState;
#[derive( Deserialize, Serialize, Debug)]
pub struct NewCustomer {
    pub username: String,
    pub password: String
}

#[post("/customers")]
pub async fn create_customer(
    body: web::Json<NewCustomer>,
    data: web::Data<AppState>
) -> impl Responder {

    let user_id: Uuid = Uuid::new_v4();
    let username = body.username.clone();
    let password = body.password.clone();

    let query_result = sqlx::query!(r#"INSERT INTO customers (id,username,password) VALUES ($1,$2,$3)"#,
    user_id,
    username,
    password
    )
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());


    if let Err(err) = query_result {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    let query_result = sqlx::query_as!(Customer, r#"SELECT * FROM customers where id = $1"#, user_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
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
pub async fn get_customers(
    data: web::Data<AppState>
) -> impl Responder {
    let query_result = sqlx::query_as!(Customer, r#"SELECT * FROM customers"#)
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(customer) => {
            return HttpResponse::Ok().json(customer)
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/customers/{user_id}")]
pub async fn get_customer(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    let query_result = sqlx::query_as!(Customer, r#"SELECT * FROM customers WHERE id = $1"#, user_id)
    .fetch_one(&data.db)
    .await;

    match query_result {
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
pub async fn delete_customer(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    let query_result = sqlx::query!(r#"DELETE FROM customers WHERE id = $1"#, user_id)
    .execute(&data.db)
    .await;


    match query_result {
        Ok(_result) => {
            return HttpResponse::Ok().json(serde_json::json!({"message": "ok"}));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}