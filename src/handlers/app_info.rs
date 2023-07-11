use crate::models::app_info as app_model;
use actix_web::{web, HttpResponse};
use sqlx::{self, Pool};
use sqlx::MySql;
use log::{ info, error };

pub async fn add_app_info(
    pool: web::Data<Pool<MySql>>,
    data: web::Json<app_model::CreateAppInfo>,
) -> HttpResponse {

    data.insert_data(&pool).await;
    info!("add_app_info");
    
    HttpResponse::Created().body("app created successfully") 
}

pub async fn get_app_info_list(
    pool: web::Data<Pool<MySql>>,
    data: web::Json<app_model::AppInfoLst>,
) -> HttpResponse {

    let result = data.list(&pool).await;
    info!("add_app_list");

    match result {
        Ok(r) => {
            HttpResponse::Created().json(r)
        }
        Err(e) => {
            error!("错误");
            HttpResponse::Created().body("query err") 
        }
    }
}