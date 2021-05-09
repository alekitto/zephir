use crate::err::ZephirError;
use actix_web::{get, web, HttpResponse};
use serde_json::Value;
use sqlx::PgPool;

#[get("/_status")]
pub(crate) async fn get_status(db_pool: web::Data<PgPool>) -> Result<HttpResponse, ZephirError> {
    let pool = db_pool.get_ref();
    sqlx::query("SELECT 1").fetch_one(pool).await?;

    Ok(HttpResponse::Ok().json(Value::from("OK")))
}
