use sqlx::{ Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_postgres_pool(
    database_url: String
) -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
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
        }
}