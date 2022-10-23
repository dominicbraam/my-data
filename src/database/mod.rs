use diesel::{
        prelude::*,
        r2d2::{self,ConnectionManager},
    };
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn create_pooled_connection() -> DbPool {
//pub fn create_pooled_connection() -> Result<DbPool, crate::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL name is incorrect");
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    //let pool = r2d2::Pool::builder()
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")

    //Ok(pool)
}
