use crate::interface_adapter::gateway::task::TaskEntity;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod query;
pub mod repository;

static TASK_STORE: Lazy<Mutex<Vec<TaskEntity>>> = Lazy::new(|| Mutex::new(vec![]));
