use crate::schema::todolists::{self};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, Debug, utoipa::ToSchema, utoipa::ToResponse)]
#[diesel(table_name = todolists)]
pub struct TodoListDTO {
    pub shared_with: Option<String>,
    pub parent_list_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListTitleDTO {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListDescriptionDTO {
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListSharedWithDTO {
    pub shared_with: String,
}

#[derive(Debug, Serialize, Deserialize, utoipa::IntoParams, utoipa::ToSchema, utoipa::ToResponse)]
pub struct UpdateTodoListParentListIdDTO {
    pub parent_list_id: i32,
}