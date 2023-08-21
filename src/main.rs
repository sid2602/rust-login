mod customers;

use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use customers::endpoints::{create_customer,get_customer,get_customers,delete_customer};
#[derive( Deserialize, Serialize, Debug)]
pub struct NewCustomer {
    pub username: String,
    pub password: String
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