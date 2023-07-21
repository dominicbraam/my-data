use diesel::prelude::*;

use crate::models::person::{NewPerson,Person};
use crate::schema::persons::dsl::*;
use crate::error::AppError;

pub fn push_person(conn: &mut PgConnection, input: crate::web::Json<NewPerson>) -> Result<usize,AppError> {

    let new_person = NewPerson {
        username: input.username.clone(),
        first_name: input.first_name.clone(),
        last_name: input.last_name.clone(),
        dob: input.dob.clone(),
        password_hash: input.password_hash.clone(),
    };

     let result = diesel::insert_into(persons)
         .values(&new_person)
         .execute(conn)
         .expect("Error creating new person");

     Ok(result)
}

pub fn pull_persons(conn: &mut PgConnection) -> Result<Vec<Person>,AppError> {
    
     let results = persons
         .load::<Person>(conn)
         .expect("Error loading persons.");

     Ok(results)
}
