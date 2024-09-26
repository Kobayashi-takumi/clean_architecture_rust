use task::TaskMemoryQuery;

pub mod task;

pub struct MemoryQueryFactory {
    task_query: TaskMemoryQuery,
}

impl MemoryQueryFactory {
    pub fn new() -> Self {
        Self {
            task_query: TaskMemoryQuery::new(),
        }
    }

    pub fn task_query(&self) -> Box<TaskMemoryQuery> {
        Box::new(self.task_query.clone())
    }
}
