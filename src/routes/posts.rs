use actix_web::web::{scope, ServiceConfig};

use crate::models::posts::{create_post, get_post_by_id, get_posts, update_post};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/posts")
            .service(get_posts)
            .service(create_post)
            .service(update_post)
            .service(get_post_by_id),
    );
}
