use super::super::{
    gateway::task::{TaskQuery, TaskRepository},
    port::task::{LoadTaskPort, LoadTasksPort, SaveTaskPort, SearchTasksPort, TaskDto},
};
use crate::shared::error::{Error, Result};
use async_trait::async_trait;
use std::str::FromStr;
use uuid::Uuid;

pub struct TaskPersistenceAdapter {
    task_repository: Box<(dyn TaskRepository + Send + Sync)>,
    task_query: Box<(dyn TaskQuery + Send + Sync)>,
}

impl TaskPersistenceAdapter {
    pub fn new(
        task_repository: Box<(dyn TaskRepository + Send + Sync)>,
        task_query: Box<(dyn TaskQuery + Send + Sync)>,
    ) -> Self {
        Self {
            task_repository,
            task_query,
        }
    }
}

impl Clone for TaskPersistenceAdapter {
    fn clone(&self) -> Self {
        Self {
            task_repository: self.task_repository.clone_box(),
            task_query: self.task_query.clone_box(),
        }
    }
}

#[async_trait]
impl LoadTaskPort for TaskPersistenceAdapter {
    async fn load_item(&self, id: &str) -> Result<TaskDto> {
        Ok(self
            .task_repository
            .get(&Uuid::from_str(id).map_err(|e| {
                log::error!("{e}");
                Error::InvalidFormat("Id".to_string())
            })?)
            .await?
            .into())
    }
    fn clone_box(&self) -> Box<dyn LoadTaskPort + Send + Sync> {
        Box::new(self.clone())
    }
}

#[async_trait]
impl LoadTasksPort for TaskPersistenceAdapter {
    async fn load_items(&self) -> Result<Vec<TaskDto>> {
        Ok(self
            .task_repository
            .list()
            .await?
            .into_iter()
            .map(|t| t.into())
            .collect())
    }
    fn clone_box(&self) -> Box<dyn LoadTasksPort + Send + Sync> {
        Box::new(self.clone())
    }
}

#[async_trait]
impl SaveTaskPort for TaskPersistenceAdapter {
    async fn save(&self, model: TaskDto) -> Result<()> {
        self.task_repository.save(model.try_into()?).await
    }
    fn clone_box(&self) -> Box<dyn SaveTaskPort + Send + Sync> {
        Box::new(self.clone())
    }
}

#[async_trait]
impl SearchTasksPort for TaskPersistenceAdapter {
    async fn search(&self, id: &Option<String>) -> Result<Vec<TaskDto>> {
        Ok(self
            .task_query
            .list(&match id {
                Some(id) => Some(Uuid::from_str(id).map_err(|e| {
                    log::error!("{e}");
                    Error::InvalidFormat("Id".to_string())
                })?),
                _ => None,
            })
            .await?
            .into_iter()
            .map(|t| t.into())
            .collect())
    }
    fn clone_box(&self) -> Box<dyn SearchTasksPort + Send + Sync> {
        Box::new(self.clone())
    }
}
