use super::super::TASK_STORE;
use crate::{
    interface_adapter::gateway::task::{TaskEntity, TaskQuery},
    shared::error::{Error, Result},
};
use std::sync::MutexGuard;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskMemoryQuery;

impl TaskMemoryQuery {
    pub fn new() -> Self {
        Self
    }
}

impl TaskMemoryQuery {
    fn store<'a>() -> Result<MutexGuard<'a, Vec<TaskEntity>>> {
        TASK_STORE.lock().map_err(|e| {
            log::error!("{e}");
            Error::Unknown
        })
    }
}

#[async_trait::async_trait]
impl TaskQuery for TaskMemoryQuery {
    async fn get(&self, id: &Uuid) -> Result<TaskEntity> {
        let store = Self::store()?;
        let task = store.iter().find(|t| &t.id == id).ok_or(Error::NotFound)?;
        Ok(task.clone())
    }
    async fn list(&self, id: &Option<Uuid>) -> Result<Vec<TaskEntity>> {
        let mut store = Self::store()?.clone();
        if let Some(id) = id {
            store.retain(|t| &t.id == id);
        }
        Ok(store)
    }
    fn clone_box(&self) -> Box<dyn TaskQuery + Send + Sync> {
        Box::new(self.clone())
    }
}
