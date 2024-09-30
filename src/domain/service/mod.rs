pub mod task;

use crate::interface_adapter::port::task::{
    LoadTaskPort, LoadTasksPort, SaveTaskPort, SearchTasksPort,
};
use task::{create::CreateTaskService, list::TasksService, update::UpdateTaskService};

#[derive(Clone)]
pub struct ServiceFactory {
    tasks_service: TasksService,
    create_task_service: CreateTaskService,
    update_task_service: UpdateTaskService,
}

impl ServiceFactory {
    pub fn new(
        search_tasks_port: Box<(dyn SearchTasksPort + Send + Sync)>,
        load_task_port: Box<(dyn LoadTaskPort + Send + Sync)>,
        load_tasks_port: Box<(dyn LoadTasksPort + Send + Sync)>,
        save_task_port: Box<(dyn SaveTaskPort + Send + Sync)>,
    ) -> Self {
        ServiceFactory {
            tasks_service: TasksService::new(search_tasks_port.clone_box()),
            create_task_service: CreateTaskService::new(
                load_tasks_port.clone_box(),
                save_task_port.clone_box(),
            ),
            update_task_service: UpdateTaskService::new(
                load_task_port,
                load_tasks_port,
                save_task_port,
            ),
        }
    }

    pub fn tasks_service(&self) -> Box<TasksService> {
        Box::new(self.tasks_service.clone())
    }
    pub fn create_task_service(&self) -> Box<CreateTaskService> {
        Box::new(self.create_task_service.clone())
    }
    pub fn update_task_service(&self) -> Box<UpdateTaskService> {
        Box::new(self.update_task_service.clone())
    }
}
