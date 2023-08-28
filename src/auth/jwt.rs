use actix_web::cookie::{Cookie, time::Duration as CookieDuration};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_token(
    user_id: Uuid,
    jwt_secret: &String
) -> String {

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;

    let claims = TokenClaims { sub: user_id.to_string(), iat, exp };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())).unwrap()
}

pub fn set_token_in_cookies(token: String) -> Cookie<'static> {
    Cookie::build("token", token)
    .path("/")
    .max_age(CookieDuration::new(60*60, 0))
    .http_only(true)
    .finish()
}

pub fn remove_token_from_cookies() -> Cookie<'static> {
    Cookie::build("token", "")
    .path("/")
    .max_age(CookieDuration::new(-1, 0))
    .http_only(true)
    .finish()
}