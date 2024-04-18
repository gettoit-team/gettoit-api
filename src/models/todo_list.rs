use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoList {
    pub id: i32,
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = todolists)]
pub struct NewTodoList<'a> {
    pub user_id: i32,
    pub shared_with: Option<&'a str>,
    pub name: &'a str,
    pub description: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTodoList {
    pub user_id: i32,
    pub shared_with: Option<String>,
    pub name: String,
    pub description: Option<String>,
}