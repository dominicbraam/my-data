#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate actix_web;

use actix_web::{middleware,web,App,HttpServer};

mod schema;
mod models;
mod database;
mod ops;
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
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
            .service(api::user::grab_users)
            .service(api::user::create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
