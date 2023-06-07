use crate::models::app_info as app_model;
use actix_web::{web, HttpResponse};
use sqlx::{self, Pool};
use sqlx::MySql;
use log::{ info };

pub async fn add_app_info(
    pool: web::Data<Pool<MySql>>,
    data: web::Json<app_model::CreateAppInfo>,
) -> HttpResponse {

    data.insert_data(&pool).await;
    info!("add_app_info");
    
    HttpResponse::Created().body("app created successfully") 
}