use std::fmt;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Error)]
pub struct ErrorResponse {
    pub status: ErrorStatus,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"status: {}, message:{}",self.status.to_string(),self.message)
    }
}
impl ErrorResponse {
    fn to_json(&self) -> serde_json::Value{
        serde_json::json!({"status": self.status.to_string(), "message": self.message})
    }
}

#[derive(Debug, Display)]
pub enum ErrorStatus {

    #[display(fmt = "BadRequest")]
    BadRequest,

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Forbidden")]
    Forbidden,

    #[display(fmt = "NotFound")]
    NotFound,

    #[display(fmt = "InternalServerError")]
    InternalServerError,
}

impl error:: ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .json(self.to_json())
    }

    fn status_code(&self) -> StatusCode {
        let status_code = &self.status;

        match status_code {
            ErrorStatus::BadRequest  => StatusCode::BAD_REQUEST,
            ErrorStatus::Unauthorized  => StatusCode::UNAUTHORIZED,
            ErrorStatus::Forbidden => StatusCode::FORBIDDEN,
            ErrorStatus::NotFound => StatusCode::NOT_FOUND,
            ErrorStatus::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}