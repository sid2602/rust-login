use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive( Deserialize, Serialize, Debug)]
pub struct Customer {
    pub id: Uuid,
    pub username: String,
    pub password: String
}