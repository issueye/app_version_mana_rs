use actix_web::HttpResponse;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("server is ok");
    HttpResponse::Ok().json(&response)
}
