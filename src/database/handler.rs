use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager
    },
};
use crate::middlewares::env_vars::get_env_var;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseHandler {
    db_host: String,
    db_port: String,
    db_name: String,
    db_user: String,
    db_password: String,
}

impl DatabaseHandler {
    pub fn new_from_env() -> Self {
        DatabaseHandler {
            db_host: get_env_var::<String>("DB_HOST", Some("localhost".to_string())),
            db_port: get_env_var::<String>("DB_PORT", Some("5432".to_string())),
            db_name: get_env_var::<String>("DB_NAME", None),
            db_user: get_env_var::<String>("DB_USER", None),
            db_password: get_env_var::<String>("DB_PASSWORD", None),
        }
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
