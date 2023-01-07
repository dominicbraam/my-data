use diesel::prelude::*;

use crate::models::person::{NewPerson,Person,InputPersonHandler};
use crate::schema::person::dsl::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn push_person(conn: &mut PgConnection, input: crate::web::Json<InputPersonHandler>) -> Result<usize,DbError> {

    let new_person = NewPerson {
        username: &input.username,
        first_name: &input.first_name,
        last_name: &input.last_name,
        dob: chrono::NaiveDate::from_ymd(1998,10,29),
    };

     let result = diesel::insert_into(person)
         .values(&new_person)
         //.get_result(conn)?;
         .execute(conn)
         .expect("Error creating new person");

     Ok(result)
}

pub fn pull_persons(conn: &mut PgConnection) -> Result<Vec<Person>,DbError> {
    
     let results = person
         .load::<Person>(conn)
         .expect("Error loading persons.");
         //.optional()?;

     Ok(results)
}
