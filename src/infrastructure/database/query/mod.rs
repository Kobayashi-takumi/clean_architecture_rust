use sqlx::PgPool;
use task::TaskDatabaseQuery;

pub mod task;

pub struct DatabaseQueryFactory {
    task_query: TaskDatabaseQuery,
}

impl DatabaseQueryFactory {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            task_query: TaskDatabaseQuery::new(pool.clone()),
        }
    }

    pub fn task_query(&self) -> Box<TaskDatabaseQuery> {
        Box::new(self.task_query.clone())
    }
}
