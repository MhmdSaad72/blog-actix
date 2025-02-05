use actix_web::web::{scope, ServiceConfig};

use crate::models::posts::{create_post, get_posts};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api").service(scope("/posts").service(get_posts).service(create_post)));
}
