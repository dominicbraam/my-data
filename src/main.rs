#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate actix_web;
extern crate reqwest;

use actix_web::{middleware,web,App,HttpServer};
use std::env;

mod schema;
mod models;
mod database;
mod endpoints;
mod error;
mod middlewares;

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
    let pool = database::create_pooled_connection();
    
    let api_url = env::var("API_URL").expect("Failed to get API URL");
    let api_port: u16 = env::var("API_PORT").expect("Failed to get API port")
        .trim()
        .parse()
        .expect("Not an integer");

    tracing::info!("starting HTTP server at http://{}:{}", api_url, api_port);

    // Start HTTP server
    HttpServer::new(move || {
/*         let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost");
 */
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            // .wrap(cors)
            // .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(middleware::Logger::default())
            .service(endpoints::person::get_person_by_id)
            .service(endpoints::person::create_person)
            .service(endpoints::finance::get_transaction_by_id)
            .service(endpoints::finance::create_transaction)
            .service(endpoints::finance::get_bank_accounts)
            .service(endpoints::finance::create_bank_account)
            .service(endpoints::finance::get_bank_branches)
            .service(endpoints::finance::get_currencies)
            .service(endpoints::protected)
    })
    .bind((api_url, api_port))?
    .run()
    .await

}
