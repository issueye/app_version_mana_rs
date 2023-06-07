use crate::handlers::handlers;
use actix_web::web::{self, get};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("health", get().to(handlers::health_check_handler));
}