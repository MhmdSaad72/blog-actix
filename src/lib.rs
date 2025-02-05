// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate serde_derive;

mod errors;
mod models;
mod routes;
mod schema;

use std::io::Result;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
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
                .app_data(pool.clone())
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
        })
        .bind(address)?
        .workers(8)
        .run()
        .await
    }
}
