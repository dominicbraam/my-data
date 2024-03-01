use diesel::prelude::*;

use crate::models::finance::{Transaction,BankAccount,BankBranch,Currency};
use crate::schema::financial::transactions::dsl::*;
use crate::schema::financial::bank_accounts::dsl::*;
use crate::schema::financial::bank_branches::dsl::*;
use crate::schema::financial::currencies::dsl::*;
use crate::error::AppError;

pub fn pull_transactions(conn: &mut PgConnection) -> Result<Vec<Transaction>,AppError> {

     let results = transactions
         .load::<Transaction>(conn)
         .map_err(AppError::from)?;

     Ok(results)
}

pub fn pull_bank_accounts(conn: &mut PgConnection) -> Result<Vec<BankAccount>,AppError> {

     let results = bank_accounts
         .load::<BankAccount>(conn)
         .map_err(AppError::from)?;

     Ok(results)
}

pub fn pull_bank_branches(conn: &mut PgConnection) -> Result<Vec<BankBranch>,AppError> {

     let results = bank_branches
         .load::<BankBranch>(conn)
         .expect("Error loading transactions.");

     Ok(results)
}

pub fn pull_currencies(conn: &mut PgConnection) -> Result<Vec<Currency>,AppError> {

     let results = currencies
         .load::<Currency>(conn)
         .expect("Error loading currency list.");

     Ok(results)
}
