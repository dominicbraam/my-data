pub mod person;
pub mod finance;

use crate::database::handler::DbPool;

use crate::middlewares::auth::AuthorizationService;
use actix_web::{
    post,
    HttpResponse
};

#[post("/protectedRoute")]
async fn protected(_: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().json("no")
}
