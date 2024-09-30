use crate::shared::error::Result;
use crate::{
    domain::service::ServiceFactory,
    interface_adapter::controller::task::{Task, TasksController},
};
use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let factory = ctx.data_unchecked::<ServiceFactory>();
        TasksController::new(factory.tasks_service())
            .execute()
            .await
    }
}
