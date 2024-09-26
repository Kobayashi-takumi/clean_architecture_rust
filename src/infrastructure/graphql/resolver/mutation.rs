use super::super::schema::{CreateTaskInput, Task, UpdateTaskInput};
use crate::shared::error::{Error, Result};
use crate::{
    domain::service::ServiceFactory,
    interface_adapter::port::task::{
        CreateTaskCommand, CreateTaskUsecase, TaskDto, UpdateTaskCommand, UpdateTaskUsecase,
    },
};
use async_graphql::*;

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

pub struct Mutation;

#[Object]
impl Mutation {
    async fn task_create<'a>(
        &self,
        ctx: &Context<'a>,
        request: CreateTaskInput,
    ) -> Result<Vec<Task>> {
        let factory = ctx.data_unchecked::<ServiceFactory>();
        let res = factory
            .create_task_service()
            .execute(CreateTaskCommand {
                title: request.title,
                description: request.description,
            })
            .await?;
        res.into_iter().map(|r| r.try_into()).collect()
    }
    async fn task_update<'a>(
        &self,
        ctx: &Context<'a>,
        request: UpdateTaskInput,
    ) -> Result<Vec<Task>> {
        let factory = ctx.data_unchecked::<ServiceFactory>();
        let res = factory
            .update_task_service()
            .execute(UpdateTaskCommand {
                id: request.id,
                title: request.title,
                description: request.description,
                is_completed: request.is_completed,
            })
            .await?;
        res.into_iter().map(|r| r.try_into()).collect()
    }
}
