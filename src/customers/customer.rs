use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool, Error};
use uuid::Uuid;
#[derive( Deserialize, Serialize, Debug)]
pub struct CreateCustomerSchema {
    pub username: String,
    pub password: String
}

#[derive( Deserialize, Serialize, Debug, Clone)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub password: String
}

pub async fn get_customer(
    user_id: Uuid,
    pg_pool: &Pool<Postgres>
) -> Result<Customer, Error> {
    sqlx::query_as!(Customer, r#"SELECT * FROM customers WHERE id = $1"#, user_id)
    .fetch_one(pg_pool)
    .await
}

pub async fn get_customer_by_username(
    username: String,
    pg_pool: &Pool<Postgres>
) -> Result<Customer, Error> {
    sqlx::query_as!(Customer, r#"SELECT * FROM customers WHERE username = $1"#, username)
    .fetch_one(pg_pool)
    .await
}

pub async fn get_customers(
    pg_pool: &Pool<Postgres>
) -> Result<Vec<Customer>, Error> {
    sqlx::query_as!(Customer, r#"SELECT * FROM customers"#)
    .fetch_all(pg_pool)
    .await
}

pub async fn create_customer(
    customer_data: CreateCustomerSchema,
    pg_pool: &Pool<Postgres>
) -> Result<Customer, Error> {

    let user_id: Uuid = Uuid::new_v4();
    let username = customer_data.username.clone();
    let password = customer_data.password.clone();

    let query_result = sqlx::query!(r#"INSERT INTO customers (id,username,password) VALUES ($1,$2,$3)"#,
    user_id,
    username,
    password
    )
        .execute(pg_pool)
        .await;


    if let Err(e) = query_result {
        return Err(e);
    }

    get_customer(user_id,pg_pool).await
}

pub async fn delete_customer(
    user_id: Uuid,
    pg_pool: &Pool<Postgres>
) -> Result<String, Error> {
    let query_result = sqlx::query!(r#"DELETE FROM customers WHERE id = $1"#, user_id)
    .execute(pg_pool)
    .await;

    if let Err(e) = query_result {
        return Err(e);
    }

    return Ok(String::from("ok"));
}