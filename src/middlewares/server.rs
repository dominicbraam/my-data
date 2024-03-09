use dotenvy::dotenv;
use std::env;
use actix_web::web;
use crate::endpoints;
use crate::database::handler::DbPool;

pub struct ServerConfig {
    pub api_host: String,
    pub api_port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        // sets env vars from .env file
        // good for dev but for prod, opt for setting the vars directly
        dotenv().ok();

        let api_host = env::var("API_HOST").unwrap_or_else(|_| "localhost".to_string());
        let api_port = env::var("API_PORT").unwrap_or_else(|_| "5432".to_string())
            .trim()
            .parse()
            .expect("Not an integer");

        ServerConfig { api_host, api_port }
    }
}

#[derive(Clone)]
pub struct AppMiddleware {
    db_pool: DbPool,
}

impl AppMiddleware {
    pub fn new(db_pool: DbPool) -> Self {
        AppMiddleware { db_pool }
    }

    pub fn configure_app(&self, cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("")
                .configure(|cfg| self.init_routes(cfg))
                .configure(|cfg| self.init_server(cfg))
        );
    }

    fn init_server(&self, cfg:  &mut web::ServiceConfig) {
        cfg.app_data(web::Data::new(self.db_pool.clone()));
    }

    fn init_routes(&self, cfg:  &mut web::ServiceConfig) {
        // register routes
        // cfg.route("/person/{uid}", web::get().to(|| endpoints::person::get_person_by_id));

        // person
        cfg.service(web::resource("/person/{uid}")
            .route(web::get().to(endpoints::person::get_person_by_id))
        );
        cfg.service(web::resource("/person")
            .route(web::post().to(endpoints::person::create_person))
        );

        // finance
        cfg.service(web::resource("/finance/transaction/{tid}")
            .route(web::get().to(endpoints::finance::get_transaction_by_id))
        );
        cfg.service(web::resource("/finance/transaction")
            .route(web::post().to(endpoints::finance::create_transaction))
        );
        cfg.service(web::resource("/finance/account")
            .route(web::get().to(endpoints::finance::get_bank_accounts))
        );
        cfg.service(web::resource("/finance/account")
            .route(web::post().to(endpoints::finance::create_bank_account))
        );
        cfg.service(web::resource("/finance/branch")
            .route(web::get().to(endpoints::finance::get_bank_branches))
        );
        cfg.service(web::resource("/finance/currency")
            .route(web::get().to(endpoints::finance::get_currencies))
        );
    }
}
