use actix_web::{
    web,
    HttpResponse,
};
use crate::database::handler::DbPool;
use crate::database::ops;
use crate::models::finance::{Transaction,NewTransaction,BankAccount,NewBankAccount};
use crate::schema::financial::{
    transactions::dsl::*,
    bank_accounts::dsl::*,
};

use crate::error::AppError;

pub async fn get_transaction_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    ) -> Result<HttpResponse, AppError>
{
    let tid = path.into_inner();

    let results: Transaction = web::block(move || {
        let mut conn = pool.get()?;
        ops::fetch_item_by_pk(&mut conn, transactions, tid)
    })
    .await?
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn create_transaction(pool: web::Data<DbPool>, body: web::Json<NewTransaction>) -> Result<HttpResponse, AppError> {

    let result: Transaction = web::block(move || {
        let mut conn = pool.get()?;
        ops::insert_into_table(&mut conn, transactions, body.into_inner())
    })
    .await?
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_bank_accounts(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let results = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_bank_accounts(&mut conn)
    })
    .await
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

pub async fn create_bank_account(pool: web::Data<DbPool>, body: web::Json<NewBankAccount>) -> Result<HttpResponse, AppError> {

    let result: BankAccount = web::block(move || {
        let mut conn = pool.get()?;
        ops::insert_into_table(&mut conn, bank_accounts, body.into_inner())
    })
    .await?
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_bank_branches(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let bank_branches = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_bank_branches(&mut conn)
    })
    .await
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(bank_branches))
}

pub async fn get_currencies(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {

    let currencies = web::block(move || {
        let mut conn = pool.get()?;
        ops::finance::pull_currencies(&mut conn)
    })
    .await
    .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(currencies))
}
