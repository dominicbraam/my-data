use core::panic;
use tracing_subscriber::fmt::format::Format;
use tracing_appender::rolling::{
    RollingFileAppender,
    Rotation,
};
use crate::middlewares::env_vars::get_env_var;

pub struct LogConfig {
    dir: String,
    file_name: String,
    log_level: u8,
    log_more: bool,
}

impl LogConfig {
    pub fn from_env() -> Self {
        LogConfig {
            dir: get_env_var::<String>("LOG_DIR", Some("./logs".to_string())),
            file_name: get_env_var::<String>("LOG_FILE", Some("log".to_string())),
            log_level: get_env_var::<u8>("LOG_LEVEL", Some(3)),
            log_more: get_env_var::<bool>("LOG_MORE", Some(false)),
        }
    }
}

pub struct LogHandler {
    config: LogConfig,
}

impl LogHandler {
    pub fn new(config: LogConfig) -> Self {
        LogHandler { config }
    }

    pub fn init_logging(&self) {

        let format = self.create_log_format();
        let log_level = self.get_log_level();
        let file_appender = self.create_file_appender(
            self.config.dir.clone(),
            self.config.file_name.clone()
        );

        // Create a `fmt` subscriber that uses our custom event format, and set it as the default.
        tracing_subscriber::fmt()
            .event_format(format)
            .with_max_level(log_level)
            .with_writer(file_appender)
            .init();
    }

    fn create_log_format(&self) -> Format {
        // Configure a custom event formatter - Logging
        tracing_subscriber::fmt::format()
            .with_thread_ids(self.config.log_more) // include the thread ID of the current thread
            .with_thread_names(self.config.log_more) // include the name of the current thread
    }

    fn create_file_appender(&self, dir: String, file_name: String) -> RollingFileAppender {
        RollingFileAppender::new(Rotation::NEVER, dir, file_name)
    }

    fn get_log_level(&self) -> tracing::Level {
        match self.config.log_level {
            1 => tracing::Level::ERROR,
            2 => tracing::Level::WARN,
            3 => tracing::Level::INFO,
            4 => tracing::Level::DEBUG,
            5 => tracing::Level::TRACE,
            _ => panic!("Not an option. Check .env.template"),
        }
    }
}
