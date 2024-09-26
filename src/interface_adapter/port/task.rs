use crate::domain::model::task::Task;
use crate::shared::error::{Error, Result};
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl TryFrom<TaskDto> for Task {
    type Error = Error;
    fn try_from(value: TaskDto) -> std::result::Result<Self, Self::Error> {
        Task::from_repository(
            value.id,
            value.title,
            value.description,
            value.is_completed,
            value.created_at,
            value.updated_at,
        )
    }
}

impl From<Task> for TaskDto {
    fn from(value: Task) -> Self {
        Self {
            id: value.id().into(),
            title: value.title(),
            description: value.description(),
            is_completed: value.is_completed(),
            created_at: value.created_at().timestamp(),
            updated_at: value.updated_at().timestamp(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTaskCommand {
    pub title: String,
    pub description: String,
}

impl From<CreateTaskCommand> for Task {
    fn from(value: CreateTaskCommand) -> Self {
        Self::new(value.title, value.description)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTaskCommand {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchTasksCommand {
    pub id: Option<String>,
}

#[async_trait]
pub trait CreateTaskUsecase {
    async fn execute(&self, request: CreateTaskCommand) -> Result<Vec<TaskDto>>;
}

#[async_trait]
pub trait UpdateTaskUsecase {
    async fn execute(&self, request: UpdateTaskCommand) -> Result<Vec<TaskDto>>;
}

#[async_trait]
pub trait TasksUsecase {
    async fn execute(&self, request: SearchTasksCommand) -> Result<Vec<TaskDto>>;
}

#[async_trait]
pub trait SearchTasksPort {
    async fn search(&self, id: &Option<String>) -> Result<Vec<TaskDto>>;
    fn clone_box(&self) -> Box<dyn SearchTasksPort + Send + Sync>;
}

#[async_trait]
pub trait LoadTasksPort {
    async fn load_items(&self) -> Result<Vec<TaskDto>>;
    fn clone_box(&self) -> Box<dyn LoadTasksPort + Send + Sync>;
}

#[async_trait]
pub trait LoadTaskPort {
    async fn load_item(&self, id: &str) -> Result<TaskDto>;
    fn clone_box(&self) -> Box<dyn LoadTaskPort + Send + Sync>;
}

#[async_trait]
pub trait SaveTaskPort {
    async fn save(&self, model: TaskDto) -> Result<()>;
    fn clone_box(&self) -> Box<dyn SaveTaskPort + Send + Sync>;
}
