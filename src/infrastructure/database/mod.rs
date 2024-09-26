use crate::shared::{
    config::Config,
    error::{Error, Result},
};
use log::error;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::path::Path;

pub mod query;
pub mod repository;
pub mod transaction;

pub async fn pool(config: &Config) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .map_err(|e| {
            error!("{}", e);
            e.into()
        })
}

pub async fn migration(pool: &PgPool, path: &str) -> Result<()> {
    let migrator = sqlx::migrate::Migrator::new(Path::new(&path))
        .await
        .map_err(|e| {
            error!("{}", e);
            Error::from(e)
        })?;
    migrator.run(pool).await.map_err(|e| {
        error!("{}", e);
        Error::from(e)
    })?;
    Ok(())
}
