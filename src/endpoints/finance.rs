use crate::actix_web::{web,get,post,HttpResponse};
use crate::database::ops;
use crate::models;
use super::DbPool;

use crate::error::AppError;

#[get("/finance/transaction")]
pub async fn get_transactions(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let transactions = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_transactions(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(transactions))
}

#[post("/finance/transaction")]
pub async fn create_transaction(pool: web::Data<DbPool>, body: web::Json<models::finance::NewTransaction>) -> Result<HttpResponse, AppError> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::push_transaction(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(size))
}

#[get("/finance/account")]
pub async fn get_bank_accounts(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let bank_accounts = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_bank_accounts(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(bank_accounts))
}

#[post("/finance/account")]
pub async fn create_bank_account(pool: web::Data<DbPool>, body: web::Json<models::finance::NewBankAccount>) -> Result<HttpResponse, AppError> {

    let size = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::push_bank_account(&mut conn, body)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(size))
}

#[get("/finance/branch")]
pub async fn get_bank_branches(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let bank_branches = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_bank_branches(&mut conn)
    })
    .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(bank_branches))
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
