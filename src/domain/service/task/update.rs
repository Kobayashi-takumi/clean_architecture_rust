use super::super::super::model::task::Task;
use crate::interface_adapter::port::task::{
    LoadTaskPort, LoadTasksPort, SaveTaskPort, TaskDto, UpdateTaskCommand, UpdateTaskUsecase,
};
use crate::shared::error::Result;

pub struct UpdateTaskService {
    load_task_port: Box<(dyn LoadTaskPort + Send + Sync)>,
    load_tasks_port: Box<(dyn LoadTasksPort + Send + Sync)>,
    save_task_port: Box<(dyn SaveTaskPort + Send + Sync)>,
}

impl UpdateTaskService {
    pub fn new(
        load_task_port: Box<(dyn LoadTaskPort + Send + Sync)>,
        load_tasks_port: Box<(dyn LoadTasksPort + Send + Sync)>,
        save_task_port: Box<(dyn SaveTaskPort + Send + Sync)>,
    ) -> Self {
        Self {
            load_task_port,
            load_tasks_port,
            save_task_port,
        }
    }
}

impl Clone for UpdateTaskService {
    fn clone(&self) -> Self {
        Self {
            load_task_port: self.load_task_port.clone_box(),
            load_tasks_port: self.load_tasks_port.clone_box(),
            save_task_port: self.save_task_port.clone_box(),
        }
    }
}

#[async_trait::async_trait]
impl UpdateTaskUsecase for UpdateTaskService {
    async fn execute(&self, request: UpdateTaskCommand) -> Result<Vec<TaskDto>> {
        let model = self.load_task_port.load_item(request.id.as_str()).await?;
        let model: Task = model.try_into()?;
        let model = model
            .change_title(request.title)
            .change_description(request.description)
            .change_is_completed(request.is_completed);
        self.save_task_port.save(model.into()).await?;
        self.load_tasks_port.load_items().await
    }
}
