use super::transaction::PgTx;
use task::TaskDatabaseRepository;

pub mod task;

pub struct DatabaseRepositoryFactory {
    task_repository: TaskDatabaseRepository,
}

impl DatabaseRepositoryFactory {
    pub fn new(tx: &PgTx) -> Self {
        Self {
            task_repository: TaskDatabaseRepository::new(tx.clone()),
        }
    }

    pub fn task_repository(&self) -> Box<TaskDatabaseRepository> {
        Box::new(self.task_repository.clone())
    }
}
