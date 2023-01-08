pub mod person;
pub mod finance;

type DbError = Box<dyn std::error::Error + Send + Sync>;
