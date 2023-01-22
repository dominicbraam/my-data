use crate::actix_web::{web,get,post,HttpResponse};
use crate::database::ops;
use crate::models;
use super::DbPool;

use crate::error::AppError;

#[get("/finance/transaction")]
pub async fn get_transactions(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let incexps = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_incexps(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(incexps))
}

#[post("/finance/transaction")]
pub async fn create_transaction(pool: web::Data<DbPool>, body: web::Json<models::finance::InputTransactionHandler>) -> Result<HttpResponse, AppError> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::push_incexp(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(size))
}

#[get("/finance/currency")]
pub async fn get_currencies(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let currencies = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_currencies(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(currencies))
}
