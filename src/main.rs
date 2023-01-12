#[macro_use]
extern crate diesel;
extern crate dotenvy;
extern crate chrono;
extern crate actix_web;
extern crate reqwest;

use actix_web::{middleware,web,App,HttpServer};

mod schema;
mod models;
mod database;
mod endpoints;
mod external;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let pool = database::create_pooled_connection();

    log::info!("starting HTTP server at http://localhost:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(endpoints::person::get_persons)
            .service(endpoints::person::create_person)
            .service(endpoints::finance::get_incexps)
            .service(endpoints::finance::create_finance_incexp)
            .service(endpoints::finance::get_currencies)
            .service(endpoints::assistant::weather::get_weather)
            .service(endpoints::assistant::greet::greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
