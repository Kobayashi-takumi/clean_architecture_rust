use crate::shared::error::Result;
use task::TaskCsvQuery;

pub mod task;

pub struct CsvQueryFactory {
    task_query: TaskCsvQuery,
}

impl CsvQueryFactory {
    pub fn new() -> Result<Self> {
        Ok(Self {
            task_query: TaskCsvQuery::new()?,
        })
    }

    pub fn task_query(&self) -> Box<TaskCsvQuery> {
        Box::new(self.task_query.clone())
    }
}
