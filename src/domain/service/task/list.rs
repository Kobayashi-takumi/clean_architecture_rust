use crate::interface_adapter::port::task::{
    SearchTasksCommand, SearchTasksPort, TaskDto, TasksUsecase,
};
use crate::shared::error::Result;

pub struct TasksService {
    load_tasks_port: Box<(dyn SearchTasksPort + Send + Sync)>,
}

impl TasksService {
    pub fn new(load_tasks_port: Box<(dyn SearchTasksPort + Send + Sync)>) -> Self {
        Self { load_tasks_port }
    }
}

impl Clone for TasksService {
    fn clone(&self) -> Self {
        Self {
            load_tasks_port: self.load_tasks_port.clone_box(),
        }
    }
}

#[async_trait::async_trait]
impl TasksUsecase for TasksService {
    async fn execute(&self, request: SearchTasksCommand) -> Result<Vec<TaskDto>> {
        self.load_tasks_port.search(&request.id).await
    }
}
