use log::{debug};
use serde::{Serialize, Deserialize};
use crate::pkg::utils::id;
use chrono::prelude::*;
use sqlx::{self, Pool, MySql};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct AppInfo {
    pub id: i32,
    pub name: String,
    pub repo_url: String,
    pub mark: String,
    pub create_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAppInfo {
    pub name: String,
    pub repo_url: Option<String>,
    pub mark: Option<String>,
}

impl CreateAppInfo {
    pub async fn insert_data(&self, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
        let result = sqlx::query("INSERT INTO app_version_mana.app_info (id, name, repo_url, mark, create_at) VALUES (?, ?, ?, ?, ?)")
        .bind(id::get_uuid())
        .bind(self.name.clone())
        .bind(self.repo_url.clone())
        .bind(self.mark.clone())
        .bind(Local::now())
        .execute(pool)
        .await?;

        debug!("影响行数：{}", result.rows_affected());
        Ok(())
    }
}