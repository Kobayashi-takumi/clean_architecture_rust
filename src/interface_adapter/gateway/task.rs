use super::super::port::task::TaskDto;
use crate::shared::error::{Error, Result};
use async_trait::async_trait;
use sqlx::FromRow;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct TaskEntity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<TaskEntity> for TaskDto {
    fn from(value: TaskEntity) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            description: value.description,
            is_completed: value.is_completed,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl TryFrom<TaskDto> for TaskEntity {
    type Error = Error;
    fn try_from(value: TaskDto) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value.id).map_err(|e| {
                log::error!("{e}");
                Error::InvalidFormat("Id".to_string())
            })?,
            title: value.title,
            description: value.description,
            is_completed: value.is_completed,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

#[async_trait]
pub trait TaskRepository {
    async fn get(&self, _id: &Uuid) -> Result<TaskEntity>;
    async fn list(&self) -> Result<Vec<TaskEntity>>;
    async fn save(&self, _model: TaskEntity) -> Result<()>;
    fn clone_box(&self) -> Box<dyn TaskRepository + Send + Sync>;
}

#[async_trait]
pub trait TaskQuery {
    async fn get(&self, _id: &Uuid) -> Result<TaskEntity>;
    async fn list(&self, _id: &Option<Uuid>) -> Result<Vec<TaskEntity>>;
    fn clone_box(&self) -> Box<dyn TaskQuery + Send + Sync>;
}
