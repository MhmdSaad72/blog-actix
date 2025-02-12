// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate serde_derive;

mod errors;
mod models;
mod routes;
mod schema;

use std::io::Result;

use actix_web::error::JsonPayloadError;
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use errors::AppError;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Blog { port }
    }

    pub async fn run(&self, database_url: String) -> Result<()> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let address = format!("127.0.0.1:{}", self.port);
        println!("Starting server at {}", address);
        HttpServer::new(move || {
            App::new()
                .app_data(web::JsonConfig::default().error_handler(json_error_handler))
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
                .configure(routes::posts::configure)
        })
        .bind(address)?
        .workers(8)
        .run()
        .await
    }
}

pub fn json_error_handler(
    err: JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let error_message = AppError::from(err);
    error_message.into()
}
