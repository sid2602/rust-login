mod customers;
mod config;
mod auth;

use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv::dotenv;
use customers::endpoints::{create_customer_endpoint,get_customer_endpoint,get_customers_endpoint,delete_customer_endpoint};
use crate::{config::postgres::create_postgres_pool, config::config::Config, auth::endpoints::{register_customer_endpoint, login_customer_endpoint, logout_customer_endpoint, get_me_endpoint}};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::init();
    let pool = create_postgres_pool(config.clone().database_url).await;

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {db: pool.clone(), env: config.clone() }))
            .service(create_customer_endpoint)
            .service(get_customers_endpoint)
            .service(get_customer_endpoint)
            .service(delete_customer_endpoint)
            .service(register_customer_endpoint)
            .service(login_customer_endpoint)
            .service(logout_customer_endpoint)
            .service(get_me_endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}