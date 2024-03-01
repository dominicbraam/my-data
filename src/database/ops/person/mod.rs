use diesel::prelude::*;

use crate::models::person::Person;
use crate::schema::persons::dsl::*;
use crate::error::AppError;

pub fn pull_persons(conn: &mut PgConnection) -> Result<Vec<Person>,AppError> {
     let results = persons
         .load::<Person>(conn)
         .map_err(AppError::from)?;

     Ok(results)
}
