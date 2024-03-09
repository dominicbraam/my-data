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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Configure a custom event formatter - Logging
    let format = tracing_subscriber::fmt::format()
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true); // include the name of the current thread

    // Create a `fmt` subscriber that uses our custom event format, and set it as the default.
    tracing_subscriber::fmt()
        .event_format(format)
        // Logger set to debug for dev; can change to info for prod
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // set up database connection pool
    let db_handler = DatabaseHandler::new();
    let pool = db_handler.create_pooled_conn();
    
    let server_conf = ServerConfig::from_env();

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
