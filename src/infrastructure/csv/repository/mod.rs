use crate::shared::error::Result;
use task::TaskCsvRepository;

pub mod task;

pub struct CsvRepositoryFactory {
    task_repository: TaskCsvRepository,
}

impl CsvRepositoryFactory {
    pub fn new() -> Result<Self> {
        Ok(Self {
            task_repository: TaskCsvRepository::new()?,
        })
    }

    pub fn task_repository(&self) -> Box<TaskCsvRepository> {
        Box::new(self.task_repository.clone())
    }
}
