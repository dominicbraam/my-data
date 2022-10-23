use diesel::prelude::*;

use crate::models::{NewUser,User,InputUserHandler};
use crate::schema::user::dsl::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn push_user(conn: &mut MysqlConnection, input: crate::web::Json<InputUserHandler>) -> Result<usize,DbError> {

    let new_user = NewUser {
        username: &input.username,
        first_name: &input.first_name,
        last_name: &input.last_name,
        dob: chrono::NaiveDate::from_ymd(1998,10,29),
    };

     let result = diesel::insert_into(user)
         .values(&new_user)
         //.get_result(conn)?;
         .execute(conn)
         .expect("Error creating new user");

     Ok(result)
}

pub fn pull_users(conn: &mut MysqlConnection) -> Result<Vec<User>,DbError> {
    
     let results = user
         .load::<User>(conn)
         .expect("Error loading users.");
         //.optional()?;

     Ok(results)
}
