use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager
    },
};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseHandler {
    db_host: String,
    db_port: String,
    db_name: String,
    db_user: String,
    db_password: String,
}

impl DatabaseHandler {
    pub fn new() -> Self {
        // sets env vars from .env file
        // good for dev but for prod, opt for setting the vars directly
        dotenv().ok();

        let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let db_name = env::var("DB_NAME").expect("DB_NAME environment variable not set");
        let db_user = env::var("DB_USER").expect("DB_USER environment variable not set");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not set");

        DatabaseHandler { db_host, db_port, db_name, db_user, db_password }
    }

    pub fn create_pooled_conn(&self) -> DbPool {
        let db_url = self.get_db_url();

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    fn get_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}
