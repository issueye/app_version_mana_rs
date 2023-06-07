use actix_web::web::Data;
use actix_web::{web, App, HttpServer, middleware};
use env_logger::Env;
use log::info;
use std::io;

mod handlers;
mod routers;
mod models;
mod db;
mod pkg;

use routers::routers as other_routers;
use routers::app_info as app_routers;
use handlers::handlers as index_handler;

#[actix_web::main]
async fn main() -> io::Result<()> {

    // std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug,actix=info");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let db_pool = db::init().await;

    let app = move || App::new()
    .app_data(Data::new(db_pool.clone()))
    .wrap(middleware::Logger::default())
    .wrap(middleware::Compress::default())
    .default_service(web::route().to(index_handler::error_404))
    .configure(other_routers::general_routes)
    .configure(app_routers::app_info_router);

    info!("服务运行中...");
    HttpServer::new(app).bind("0.0.0.0:10789")?.run().await
}
