use actix_web::web::{self, ServiceConfig};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(|| async { "Create user" }))
            .route("", web::get().to(|| async { "List users" }))
            .route("/{id}", web::get().to(|| async { "Get user by id" }))
            .route("/{id}", web::put().to(|| async { "Update user" }))
            .route("/{id}", web::delete().to(|| async { "Delete user" })),
    );
}
