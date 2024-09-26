use super::super::transaction::PgTx;
use crate::{
    interface_adapter::gateway::task::{TaskEntity, TaskRepository},
    shared::error::{Error, Result},
};
use sqlx::{query, query_as};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskDatabaseRepository {
    tx: PgTx,
}

impl TaskDatabaseRepository {
    pub fn new(tx: PgTx) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl TaskRepository for TaskDatabaseRepository {
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
        .fetch_one(&mut **self.tx.lock().await)
        .await
        .map_err(|e| {
            log::error!("{e}");
            e.into()
        })
    }
    async fn list(&self) -> Result<Vec<TaskEntity>> {
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
            "#,
        )
        .fetch_all(&mut **self.tx.lock().await)
        .await
        .map_err(|e| {
            log::error!("{e}");
            e.into()
        })
    }
    async fn save(&self, model: TaskEntity) -> Result<()> {
        query(
            r#"
INSERT INTO
    tasks (
        id,
        title,
        description,
        is_completed,
        created_at,
        updated_at
    )
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6
)
ON CONFLICT
    (id)
DO UPDATE SET
    title=$2, description=$3, is_completed=$4, updated_at=$6
            "#,
        )
        .bind(model.id)
        .bind(model.title)
        .bind(model.description)
        .bind(model.is_completed)
        .bind(model.created_at)
        .bind(model.updated_at)
        .execute(&mut **self.tx.lock().await)
        .await
        .map_err(|e| {
            log::error!("{e}");
            Error::from(e)
        })?;
        Ok(())
    }
    fn clone_box(&self) -> Box<dyn TaskRepository + Send + Sync> {
        Box::new(self.clone())
    }
}
