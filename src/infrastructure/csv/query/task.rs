use super::super::{open_file, tasks_from_csv, TASK_FILE_PATH};
use crate::{
    interface_adapter::gateway::task::{TaskEntity, TaskQuery},
    shared::error::{Error, Result},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskCsvQuery {
    store: Vec<TaskEntity>,
}

impl TaskCsvQuery {
    pub fn new() -> Result<Self> {
        let file = open_file(TASK_FILE_PATH)?;
        let store = tasks_from_csv(file)?;
        Ok(Self { store })
    }
}

#[async_trait::async_trait]
impl TaskQuery for TaskCsvQuery {
    async fn get(&self, id: &Uuid) -> Result<TaskEntity> {
        let task = self
            .store
            .iter()
            .find(|t| &t.id == id)
            .ok_or(Error::NotFound)?;
        Ok(task.clone())
    }
    async fn list(&self, id: &Option<Uuid>) -> Result<Vec<TaskEntity>> {
        let mut store = self.store.clone();
        if let Some(id) = id {
            store.retain(|t| &t.id == id);
        }
        Ok(store)
    }
    fn clone_box(&self) -> Box<dyn TaskQuery + Send + Sync> {
        Box::new(self.clone())
    }
}
