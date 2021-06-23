use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use libzephir::err::Error as LibError;
use libzephir::policy::allowed_result::AllowedResult;
use libzephir::policy::policy::ToJson;
use serde_json::json;
use sqlx::error::Error as DatabaseError;
use validator::ValidationErrors;

#[derive(Display, From, Debug)]
pub enum ZephirError {
    NotFound,
    PoolError(DatabaseError),
    AllowedError,
    ValidationError(ValidationErrors),
    InvalidRequestError,

    ServerError(LibError),
}

impl std::error::Error for ZephirError {}
impl ResponseError for ZephirError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ZephirError::NotFound => HttpResponse::NotFound().json(json!({
                "status_code": 404,
                "errors": "Not found"
            })),
            ZephirError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            ZephirError::AllowedError => {
                HttpResponse::Forbidden().json(AllowedResult::denied().to_value())
            }
            ZephirError::ServerError(ref err) => HttpResponse::InternalServerError().json(json!({
                "status_code": 500,
                "errors": err.to_string()
            })),
            ZephirError::ValidationError(ref err) => HttpResponse::BadRequest().json(json!({
                "status_code": 400,
                "errors": serde_json::to_value(err.field_errors()).unwrap()
            })),
            ZephirError::InvalidRequestError => HttpResponse::BadRequest().json(json!({
                "status_code": 400,
                "errors": [
                    "Invalid request"
                ]
            })),
        }
    }
}
