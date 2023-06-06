use actix_web::{App, HttpServer};
use std::io;

pub mod handlers;
pub mod routers;

use routers::routers as other_routers;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(other_routers::general_routes);

    HttpServer::new(app).bind("0.0.0.0:10789")?.run().await
}
