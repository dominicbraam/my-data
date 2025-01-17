use serde::{
    Serialize,
    Deserialize,
};
use diesel::{
    Queryable,
    Insertable,
    Identifiable,
    AsChangeset,
};
use crate::schema::persons;

#[derive(Serialize,Deserialize,Queryable,AsChangeset,Identifiable)]
#[diesel(table_name = persons)]
pub struct Person {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: chrono::NaiveDate,
    pub password_hash: String,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = persons)]
pub struct NewPerson {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: chrono::NaiveDate,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username: String,
/*     pub password: String,
    #[serde(default)]
    pub remember_me: bool, */
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
