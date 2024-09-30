use super::Date;
use crate::interface_adapter::port::task::{
    CreateTaskCommand, CreateTaskUsecase, SearchTasksCommand, TaskDto, TasksUsecase,
    UpdateTaskCommand, UpdateTaskUsecase,
};
use crate::shared::error::{Error, Result};
use async_graphql::*;

#[derive(SimpleObject, Debug, PartialEq, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: Date,
    pub updated_at: Date,
}

impl TryFrom<TaskDto> for Task {
    type Error = Error;
    fn try_from(value: TaskDto) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            title: value.title,
            description: value.description,
            is_completed: value.is_completed,
            created_at: value.created_at.try_into()?,
            updated_at: value.updated_at.try_into()?,
        })
    }
}

#[derive(InputObject, Debug, PartialEq, Clone)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

impl From<CreateTaskRequest> for CreateTaskCommand {
    fn from(value: CreateTaskRequest) -> Self {
        Self {
            title: value.title,
            description: value.description,
        }
    }
}

#[derive(InputObject, Debug, PartialEq, Clone)]
pub struct UpdateTaskRequest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

impl From<UpdateTaskRequest> for UpdateTaskCommand {
    fn from(value: UpdateTaskRequest) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            is_completed: value.is_completed,
        }
    }
}

///
/// タスク一覧のコントローラ
///
pub struct TasksController {
    tasks_usecase: Box<(dyn TasksUsecase + Sync + Send)>,
}

impl TasksController {
    pub fn new(tasks_usecase: Box<(dyn TasksUsecase + Sync + Send)>) -> Self {
        Self { tasks_usecase }
    }

    pub async fn execute(&self) -> Result<Vec<Task>> {
        let res = self
            .tasks_usecase
            .execute(SearchTasksCommand { id: None })
            .await?;
        res.into_iter().map(|r| r.try_into()).collect()
    }
}

///
/// タスク作成のコントローラ
///
pub struct CreateTaskController {
    create_task_usecase: Box<(dyn CreateTaskUsecase + Sync + Send)>,
}

impl CreateTaskController {
    pub fn new(create_task_usecase: Box<(dyn CreateTaskUsecase + Sync + Send)>) -> Self {
        Self {
            create_task_usecase,
        }
    }

    pub async fn execute(&self, request: CreateTaskRequest) -> Result<Vec<Task>> {
        let res = self.create_task_usecase.execute(request.into()).await?;
        res.into_iter().map(|r| r.try_into()).collect()
    }
}

///
/// タスク更新のコントローラ
///
pub struct UpdateTaskController {
    create_task_usecase: Box<(dyn UpdateTaskUsecase + Sync + Send)>,
}

impl UpdateTaskController {
    pub fn new(create_task_usecase: Box<(dyn UpdateTaskUsecase + Sync + Send)>) -> Self {
        Self {
            create_task_usecase,
        }
    }

    pub async fn execute(&self, request: UpdateTaskRequest) -> Result<Vec<Task>> {
        let res = self.create_task_usecase.execute(request.into()).await?;
        res.into_iter().map(|r| r.try_into()).collect()
    }
}
