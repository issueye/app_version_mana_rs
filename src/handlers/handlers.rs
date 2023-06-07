use actix_web::HttpResponse;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("server is ok");
    HttpResponse::Ok().json(&response)
}


pub async fn error_404() -> HttpResponse {
    HttpResponse::NotFound().body("404 not found")
}