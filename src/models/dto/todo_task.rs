use chrono::NaiveDate; 
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskSummaryDTO {
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskDescriptionDTO {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskParentTaskDTO {
    pub parent_task_id: i32,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskDueDateDTO {
    pub due_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskTodoListDTO {
    pub todolist_id: i32,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoTaskDoneDTO {
    pub done: bool,
}
