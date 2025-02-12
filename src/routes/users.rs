use actix_web::web::{self, ServiceConfig};

use crate::models::users::{create_user, get_all_users, handle_json};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .service(create_user)
            .service(get_all_users)
            .service(handle_json),
    );
}
