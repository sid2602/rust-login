mod customers;
mod config;

use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv::dotenv;
use customers::endpoints::{create_customer_endpoint,get_customer_endpoint,get_customers_endpoint,delete_customer_endpoint};
use crate::config::postgres::create_postgres_pool;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_address = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_postgres_pool(db_address).await;

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {db: pool.clone()}))
            .service(create_customer_endpoint)
            .service(get_customers_endpoint)
            .service(get_customer_endpoint)
            .service(delete_customer_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}