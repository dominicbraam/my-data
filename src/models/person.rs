use serde::{Deserialize,Serialize};
use crate::schema::person;

#[derive(Serialize,Deserialize,Queryable)]
pub struct Person {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: chrono::NaiveDate,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = person)]
pub struct NewPerson<'a> {
    pub username: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub dob: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPersonHandler {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}
