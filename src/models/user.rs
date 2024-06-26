use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserFirstName {
    pub id: i32,
    pub first_name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserLastName {
    pub id: i32,
    pub last_name: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserEmail {
    pub id: i32,
    pub email: String,
    pub modified_at: NaiveDateTime,
}