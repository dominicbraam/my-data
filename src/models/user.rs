use serde::{Deserialize,Serialize};
use crate::schema::user;

#[derive(Serialize,Deserialize,Queryable)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: chrono::NaiveDate,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub dob: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUserHandler {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}
