#[macro_use]
extern crate diesel;
extern crate dotenvy;
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

    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let pool = database::create_pooled_connection();
    
    let api_url = env::var("API_URL").expect("Failed to get API URL");
    let api_port: u16 = env::var("API_PORT").expect("Failed to get API port")
        .trim()
        .parse()
        .expect("Not an integer");

    log::info!("starting HTTP server at http://localhost:8080");

    // Start HTTP server
    HttpServer::new(move || {
/*         let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost");
 */
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            // .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(endpoints::person::get_persons)
            .service(endpoints::person::create_person)
            .service(endpoints::finance::get_transactions)
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
