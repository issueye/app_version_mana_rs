use sqlx::{Pool};
use sqlx::mysql::MySqlPoolOptions;

// 数据库初始化
pub async fn init() -> Pool<sqlx::MySql> {
    let pool = MySqlPoolOptions::new().max_connections(5).connect("mysql://root:123456@127.0.0.1:3306/app_version_mana").await.unwrap();
    pool
}