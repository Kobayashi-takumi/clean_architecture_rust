use super::gateway::task::{TaskQuery, TaskRepository};
use task::TaskPersistenceAdapter;

pub mod task;

pub struct PersistenceAdapterFactory {
    task_persistence_adapter: TaskPersistenceAdapter,
}

impl PersistenceAdapterFactory {
    pub fn new(
        task_repository: Box<(dyn TaskRepository + Send + Sync)>,
        task_query: Box<(dyn TaskQuery + Send + Sync)>,
    ) -> Self {
        Self {
            task_persistence_adapter: TaskPersistenceAdapter::new(task_repository, task_query),
        }
    }
    pub fn task_persistence_adapter(&self) -> Box<TaskPersistenceAdapter> {
        Box::new(self.task_persistence_adapter.clone())
    }
}
