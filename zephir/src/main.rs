#[macro_use]
extern crate lazy_static;

mod err;
mod handlers;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use libzephir::err::{Error, ErrorKind};
use libzephir::storage::StorageManager;
use sqlx::postgres::PgPoolOptions;

fn get_serve_port() -> u16 {
    let serve_port = std::env::var("SERVE_PORT");
    match serve_port {
        Result::Err(_) => 8091,
        Result::Ok(serve_port) => {
            if serve_port.is_empty() {
                8091
            } else {
                serve_port.parse().unwrap_or(8091)
            }
        }
    }
}

fn get_db_connection_string() -> Result<String, Error> {
    match std::env::var("DSN") {
        Result::Ok(dsn) => {
            if dsn.is_empty() {
                Err(Error::new(
                    ErrorKind::UnknownError,
                    "Database DSN is empty. Please set DSN env var to a non-empty value",
                ))
            } else {
                Ok(dsn)
            }
        }
        Result::Err(_) => Err(Error::new(
            ErrorKind::UnknownError,
            "Database DSN not set. Please set DSN env var",
        )),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    libzephir::initialize_libzephir();

    let min_connections: u32 = std::env::var("MINCONN")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .unwrap();
    let max_connections: u32 = std::env::var("MAXCONN")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap();

    let pool = PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(get_db_connection_string().unwrap().as_str())
        .await
        .unwrap();

    let storage_manager = StorageManager::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(storage_manager.clone())
            .wrap(Logger::default())
            .service(handlers::get_status)
            .service(handlers::allowed_action)
            .service(handlers::get_group)
            .service(handlers::get_group_identities)
            .service(handlers::patch_group_identities)
            .service(handlers::upsert_group)
            .service(handlers::get_identity)
            .service(handlers::upsert_identity)
            .service(handlers::get_policy)
            .service(handlers::upsert_policy)
    })
    .bind(("0.0.0.0", get_serve_port()))?
    .run()
    .await
}
