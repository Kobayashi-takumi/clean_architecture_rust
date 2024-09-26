use async_graphql::*;
use scaler::date::Date;

mod scaler;

#[derive(SimpleObject, Debug, PartialEq, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: Date,
    pub updated_at: Date,
}

#[derive(InputObject, Debug, PartialEq, Clone)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: String,
}

#[derive(InputObject, Debug, PartialEq, Clone)]
pub struct UpdateTaskInput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}
