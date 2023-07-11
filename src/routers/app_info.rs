use crate::handlers::app_info as app_handlers;
use actix_web::web::{self, post, get, resource as r, scope};

pub fn app_info_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("app")
        .service(
            r("")
            .route(
                post().to(app_handlers::add_app_info)
            )
        )
    ).service(
        scope("list")
        .service(
            r("").route(
                get().to(app_handlers::get_app_info_list)
            )
        )
    );
}
