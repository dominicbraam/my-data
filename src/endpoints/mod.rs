pub mod person;
pub mod finance;
pub mod assistant;

use crate::database::DbPool;

use crate::middlewares::auth::AuthorizationService;
use crate::actix_web::{web,get,post,HttpResponse};

#[post("/protectedRoute")]
async fn protected(_: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().json("no")
}
