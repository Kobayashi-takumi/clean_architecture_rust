use crate::shared::error::Result;
use crate::{
    domain::service::ServiceFactory,
    interface_adapter::controller::task::{
        CreateTaskController, CreateTaskRequest, Task, UpdateTaskController, UpdateTaskRequest,
    },
};
use async_graphql::*;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn task_create<'a>(
        &self,
        ctx: &Context<'a>,
        request: CreateTaskRequest,
    ) -> Result<Vec<Task>> {
        let factory = ctx.data_unchecked::<ServiceFactory>();
        CreateTaskController::new(factory.create_task_service())
            .execute(request)
            .await
    }
    async fn task_update<'a>(
        &self,
        ctx: &Context<'a>,
        request: UpdateTaskRequest,
    ) -> Result<Vec<Task>> {
        let factory = ctx.data_unchecked::<ServiceFactory>();
        UpdateTaskController::new(factory.update_task_service())
            .execute(request)
            .await
    }
}
