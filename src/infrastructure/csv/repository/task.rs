use super::super::{open_file, output_csv, tasks_from_csv, TASK_FILE_PATH};
use crate::{
    interface_adapter::gateway::task::{TaskEntity, TaskRepository},
    shared::error::{Error, Result},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskCsvRepository {
    store: Vec<TaskEntity>,
}

impl TaskCsvRepository {
    pub fn new() -> Result<Self> {
        let file = open_file(TASK_FILE_PATH)?;
        let store = tasks_from_csv(file)?;
        Ok(Self { store })
    }
}

#[async_trait::async_trait]
impl TaskRepository for TaskCsvRepository {
    async fn get(&self, id: &Uuid) -> Result<TaskEntity> {
        let task = self
            .store
            .iter()
            .find(|t| &t.id == id)
            .ok_or(Error::NotFound)?;
        Ok(task.clone())
    }
    async fn list(&self) -> Result<Vec<TaskEntity>> {
        Ok(self.store.clone())
    }
    async fn save(&self, model: TaskEntity) -> Result<()> {
        let mut store = self.store.clone();
        if let Some(index) = store.iter().position(|t| t.id == model.id) {
            store[index] = model;
        } else {
            store.push(model);
        };
        output_csv(
            TASK_FILE_PATH,
            vec![
                "id".to_string(),
                "title".to_string(),
                "description".to_string(),
                "is_completed".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ],
            store
                .clone()
                .into_iter()
                .map(|t| {
                    vec![
                        t.id.to_string(),
                        t.title,
                        t.description,
                        t.is_completed.to_string(),
                        t.created_at.to_string(),
                        t.updated_at.to_string(),
                    ]
                })
                .collect(),
        )?;
        Ok(())
    }
    fn clone_box(&self) -> Box<dyn TaskRepository + Send + Sync> {
        Box::new(self.clone())
    }
}
