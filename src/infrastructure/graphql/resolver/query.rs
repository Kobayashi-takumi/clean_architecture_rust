use super::super::schema::Task;
use crate::shared::error::Result;
use crate::{
    domain::service::ServiceFactory,
    interface_adapter::port::task::{SearchTasksCommand, TasksUsecase},
};
use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        let factory = ctx.data_unchecked::<ServiceFactory>();
        let res = factory
            .tasks_service()
            .execute(SearchTasksCommand { id: None })
            .await?;
        res.into_iter().map(|r| r.try_into()).collect()
    }
}
