use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool, Error};
use uuid::Uuid;
#[derive( Deserialize, Serialize, Debug)]
pub struct CreateCustomerSchema {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub role: Option<CustomerRole>
}

#[derive(sqlx::Type, PartialEq, Deserialize, Serialize, Debug, Clone)]
#[sqlx(type_name = "customer_role", rename_all = "lowercase")]
pub enum CustomerRole {
    Admin,
    Staff,
    Viewer
}

#[derive( Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub role: CustomerRole,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn get_customer(
    user_id: Uuid,
    pg_pool: &Pool<Postgres>
) -> Result<Customer, Error> {
    sqlx::query_as!(Customer, r#"SELECT id, firstname, lastname, password, email, created_at, updated_at, role as "role: CustomerRole"  FROM customers WHERE id = $1"#, user_id)
    .fetch_one(pg_pool)
    .await
}

pub async fn get_customer_by_email(
    email: String,
    pg_pool: &Pool<Postgres>
) -> Result<Customer, Error> {
    sqlx::query_as!(Customer, r#"SELECT id, firstname, lastname, password, email, created_at, updated_at, role as "role: CustomerRole" FROM customers WHERE email = $1"#, email)
    .fetch_one(pg_pool)
    .await
}

pub async fn get_customers(
    pg_pool: &Pool<Postgres>
) -> Result<Vec<Customer>, Error> {
    sqlx::query_as!(Customer, r#"SELECT id, firstname, lastname, password, email, created_at, updated_at, role as "role: CustomerRole" FROM customers"#)
    .fetch_all(pg_pool)
    .await
}

pub async fn create_customer(
    customer_data: CreateCustomerSchema,
    pg_pool: &Pool<Postgres>
) -> Result<Customer, Error> {

    let email = customer_data.email.clone();
    let firstname = customer_data.firstname.clone();
    let lastname = customer_data.lastname.clone();
    let password = customer_data.password.clone();
    let role = customer_data.role.unwrap_or(CustomerRole::Viewer).clone();

    let query_result = sqlx::query_as!(Customer,r#"INSERT INTO customers (email,firstname,lastname,password,role) VALUES ($1,$2,$3,$4,$5) RETURNING id,email,firstname,lastname,password,created_at, updated_at,role as "role: CustomerRole""#,
    email,
    firstname,
    lastname,
    password,
    role as CustomerRole
    )
    .fetch_one(pg_pool)
    .await;

    return query_result;
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