use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use sqlx::error::Error as DatabaseError;
use libzephir::policy::allowed_result::AllowedResult;
use libzephir::policy::policy::ToJson;
use libzephir::err::Error as LibError;
use serde_json::{Map, Value};
use validator::ValidationErrors;

#[derive(Display, From, Debug)]
pub enum ZephirError {
    NotFound,
    PoolError(DatabaseError),
    AllowedError,
    ValidationError(ValidationErrors),

    ServerError(LibError),
}

impl std::error::Error for ZephirError {}
impl ResponseError for ZephirError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ZephirError::NotFound => {
                let mut map = Map::new();
                map.insert("status_code".to_string(), Value::from(404));
                map.insert("message".to_string(), Value::from("Not found"));

                HttpResponse::NotFound().json(map)
            },
            ZephirError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            ZephirError::AllowedError => {
                HttpResponse::Forbidden().json(AllowedResult::denied().to_value())
            }
            ZephirError::ServerError(ref err) => {
                let mut map = Map::new();
                map.insert("status_code".to_string(), Value::from(500));
                map.insert("error".to_string(), Value::from(err.to_string()));

                HttpResponse::InternalServerError().json(map)
            }
            ZephirError::ValidationError(ref err) => {
                let mut map = Map::new();
                map.insert("status_code".to_string(), Value::from(400));
                map.insert("errors".to_string(), serde_json::to_value(err.field_errors()).unwrap());

                HttpResponse::BadRequest().json(map)
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
