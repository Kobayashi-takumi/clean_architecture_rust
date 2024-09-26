use super::super::TASK_STORE;
use crate::{
    interface_adapter::gateway::task::{TaskEntity, TaskRepository},
    shared::error::{Error, Result},
};
use std::sync::MutexGuard;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskMemoryRepository;

impl TaskMemoryRepository {
    pub fn new() -> Self {
        Self
    }
}

impl TaskMemoryRepository {
    fn store<'a>() -> Result<MutexGuard<'a, Vec<TaskEntity>>> {
        TASK_STORE.lock().map_err(|e| {
            log::error!("{e}");
            Error::Unknown
        })
    }
}

#[async_trait::async_trait]
impl TaskRepository for TaskMemoryRepository {
    async fn get(&self, id: &Uuid) -> Result<TaskEntity> {
        let store = Self::store()?;
        let task = store.iter().find(|t| &t.id == id).ok_or(Error::NotFound)?;
        Ok(task.clone())
    }
    async fn list(&self) -> Result<Vec<TaskEntity>> {
        Ok(Self::store()?.clone())
    }
    async fn save(&self, model: TaskEntity) -> Result<()> {
        let mut store = Self::store()?;
        if let Some(index) = store.iter().position(|t| t.id == model.id) {
            store[index] = model;
        } else {
            store.push(model);
        };
        Ok(())
    }
    fn clone_box(&self) -> Box<dyn TaskRepository + Send + Sync> {
        Box::new(self.clone())
    }
}
