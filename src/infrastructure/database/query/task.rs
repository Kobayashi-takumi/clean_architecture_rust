use crate::{
    interface_adapter::gateway::task::{TaskEntity, TaskQuery},
    shared::error::Result,
};
use sqlx::PgPool;
use sqlx::{query_as, QueryBuilder};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskDatabaseQuery {
    pool: PgPool,
}

impl TaskDatabaseQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskQuery for TaskDatabaseQuery {
    async fn get(&self, id: &Uuid) -> Result<TaskEntity> {
        query_as(
            r#"
SELECT
    id,
    title,
    description,
    is_completed,
    created_at,
    updated_at
FROM
    tasks
WHERE
    id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            log::error!("{e}");
            e.into()
        })
    }
    async fn list(&self, id: &Option<Uuid>) -> Result<Vec<TaskEntity>> {
        let mut query = QueryBuilder::new(
            r#"
SELECT
    id,
    title,
    description,
    is_completed,
    created_at,
    updated_at
FROM
    tasks
WHERE
    1=1            
            "#,
        );
        if let Some(id) = id {
            query
                .push(
                    r#"
AND
    id = 
                "#,
                )
                .push_bind(id);
        }
        query
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("{e}");
                e.into()
            })
    }
    fn clone_box(&self) -> Box<dyn TaskQuery + Send + Sync> {
        Box::new(self.clone())
    }
}
