use super::super::super::model::task::Task;
use crate::interface_adapter::port::task::{
    CreateTaskCommand, CreateTaskUsecase, LoadTasksPort, SaveTaskPort, TaskDto,
};
use crate::shared::error::Result;

pub struct CreateTaskService {
    load_tasks_port: Box<(dyn LoadTasksPort + Send + Sync)>,
    save_task_port: Box<(dyn SaveTaskPort + Send + Sync)>,
}

impl CreateTaskService {
    pub fn new(
        load_tasks_port: Box<(dyn LoadTasksPort + Send + Sync)>,
        save_task_port: Box<(dyn SaveTaskPort + Send + Sync)>,
    ) -> Self {
        Self {
            load_tasks_port,
            save_task_port,
        }
    }
}

impl Clone for CreateTaskService {
    fn clone(&self) -> Self {
        Self {
            load_tasks_port: self.load_tasks_port.clone_box(),
            save_task_port: self.save_task_port.clone_box(),
        }
    }
}

#[async_trait::async_trait]
impl CreateTaskUsecase for CreateTaskService {
    async fn execute(&self, request: CreateTaskCommand) -> Result<Vec<TaskDto>> {
        let model: Task = request.into();
        self.save_task_port.save(model.into()).await?;
        self.load_tasks_port.load_items().await
    }
}
