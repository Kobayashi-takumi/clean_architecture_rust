use crate::shared::error::{Error, Result};
use log::error;
use std::env::var;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// データベースの接続先URL
    pub database_url: String,
    /// データベースの最大コネクション数
    pub max_connections: u32,
    /// データベースのマイグレーションファイルのパス
    pub migrations_path: Option<String>,
    /// 開発者モードか？
    pub dev_mode: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = var("DATABASE_URL").map_err(|e| {
            error!("{}", e);
            Error::Configuration("DATABASE_URL".to_string())
        })?;
        let migrations_path = var("DATABASE_MIGRATIONS_PATH").ok();
        let max_connections = var("DB_MAX_CONNECTIONS")
            .unwrap_or("10".to_string())
            .parse::<u32>()
            .map_err(|e| {
                log::error!("{}", e);
                Error::Configuration("DB_MAX_CONNECTIONS".to_string())
            })?;
        let dev_mode = var("DEV_MODE")
            .map(|mode| matches!(mode.as_str(), "true" | "True" | "TRUE"))
            .unwrap_or(false);
        Ok(Self {
            database_url,
            max_connections,
            migrations_path,
            dev_mode,
        })
    }
}
