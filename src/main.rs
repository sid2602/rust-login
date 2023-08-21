use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use uuid::Uuid;

#[derive( Deserialize, Serialize, Debug)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub password: String
}

#[derive( Deserialize, Serialize, Debug)]
pub struct NewCustomer {
    pub username: String,
    pub password: String
}

#[post("/customers")]
async fn create_customer(
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
async fn get_customers(
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
async fn get_customer(
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
async fn delete_customer(
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

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_address = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_address)
        .await
        {
            Ok(pool) => {
                println!("Connection to the database is successful!");
                pool
            }
            Err(error) => {
                println!("Failed to connect to the database: {:?}", error);
                std::process::exit(1);
            }
        };

        println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {db: pool.clone()}))
            .service(create_customer)
            .service(get_customers)
            .service(get_customer)
            .service(delete_customer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}