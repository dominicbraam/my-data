mod schema;
mod models;
mod database;
mod endpoints;
mod error;
mod middlewares;

use actix_web::{
    App,
    middleware,
    HttpServer,
};
use database::handler::DatabaseHandler;
use middlewares::server::ServerConfig;
use middlewares::server::AppMiddleware;
use middlewares::logging::LogConfig;
use middlewares::logging::LogHandler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let log_conf = LogConfig::new_from_env();
    LogHandler::new(log_conf)
        .init_logging();

    // set up database connection pool
    let pool = DatabaseHandler::new_from_env()
        .create_pooled_conn();
    
    let server_conf = ServerConfig::new_from_env();

    let app_middleware = AppMiddleware::new(pool);

    log::info!(
        "Starting HTTP server at http://{}:{}",
        server_conf.api_host, server_conf.api_port
    );

    // Start HTTP server
    HttpServer::new(move || {
/*         let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost");
 */
        App::new()
            // .wrap(cors)
            // .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(middleware::Logger::default())
            .configure(|cfg| app_middleware.configure_app(cfg))
    })
    .bind((server_conf.api_host, server_conf.api_port))?
    .run()
    .await

}
