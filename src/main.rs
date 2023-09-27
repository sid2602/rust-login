mod customers;
mod config;
mod auth;

use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv::dotenv;
use crate::{config::postgres::create_postgres_pool, config::config::Config, auth::endpoints::auth_scope, customers::endpoints::customer_scope};

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
            .service(auth_scope())
            .service(customer_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}