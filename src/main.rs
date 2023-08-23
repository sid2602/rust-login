mod customers;
mod config;

use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv::dotenv;
use customers::endpoints::{create_customer,get_customer,get_customers,delete_customer};
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
            .service(create_customer)
            .service(get_customers)
            .service(get_customer)
            .service(delete_customer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}