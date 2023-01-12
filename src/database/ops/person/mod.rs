use diesel::prelude::*;

use crate::models::person::{NewPerson,Person,InputPersonHandler};
use crate::schema::person::dsl::*;
use crate::error::AppError;

pub fn push_person(conn: &mut PgConnection, input: crate::web::Json<InputPersonHandler>) -> Result<usize,AppError> {

    let dateofbirth = match chrono::NaiveDate::from_ymd_opt(1998,12,29){
        Some(date) => date,
        None => {
            log::error!("Invalid date");
            panic!("Invalid date")
        }
    };

    let new_person = NewPerson {
        username: &input.username,
        first_name: &input.first_name,
        last_name: &input.last_name,
        dob: dateofbirth,
    };

     let result = diesel::insert_into(person)
         .values(&new_person)
         .execute(conn)
         .expect("Error creating new person");

     Ok(result)
}

pub fn pull_persons(conn: &mut PgConnection) -> Result<Vec<Person>,AppError> {
    
     let results = person
         .load::<Person>(conn)
         .expect("Error loading persons.");

     Ok(results)
}
