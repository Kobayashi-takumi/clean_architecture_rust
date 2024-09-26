use task::TaskMemoryRepository;

pub mod task;

pub struct MemoryRepositoryFactory {
    task_repository: TaskMemoryRepository,
}

impl MemoryRepositoryFactory {
    pub fn new() -> Self {
        Self {
            task_repository: TaskMemoryRepository::new(),
        }
    }

    pub fn task_repository(&self) -> Box<TaskMemoryRepository> {
        Box::new(self.task_repository.clone())
    }
}
