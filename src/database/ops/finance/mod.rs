use diesel::prelude::*;

use crate::models::finance::{NewTransaction,Transaction,NewBankAccount,BankAccount,BankBranch,Currency};
use crate::schema::transactions::dsl::*;
use crate::schema::bank_accounts::dsl::*;
use crate::schema::bank_branches::dsl::*;
use crate::schema::currencies::dsl::*;
use crate::error::AppError;

pub fn push_transaction(conn: &mut PgConnection, input: crate::web::Json<NewTransaction>) -> Result<usize,AppError> {

    let new_transaction = NewTransaction {
        group_id: input.group_id.clone(),
        account_id: input.account_id.clone(),
        action_id: input.action_id.clone(),
        tag_id: input.tag_id.clone(),
        product_id: input.product_id.clone(),
        document_id: input.document_id.clone(),
        is_need: input.is_need.clone(),
        amount: input.amount.clone(),
        transaction_datetime: input.transaction_datetime.clone(),
        description: input.description.clone(),
    };

     let result = diesel::insert_into(transactions)
         .values(&new_transaction)
         .execute(conn)
         .expect("Error creating new transaction.");

     Ok(result)
}

pub fn pull_transactions(conn: &mut PgConnection) -> Result<Vec<Transaction>,AppError> {

     let results = transactions
         .load::<Transaction>(conn)
         .expect("Error loading transactions.");

     Ok(results)
}

pub fn push_bank_account(conn: &mut PgConnection, input: crate::web::Json<NewBankAccount>) -> Result<usize,AppError> {

    let new_bank_account = NewBankAccount {
        person_id: input.person_id.clone(),
        account_type_id: input.account_type_id.clone(),
        currency_id: input.currency_id.clone(),
        branch_id: input.branch_id.clone(),
        account_number: input.account_number.clone(),
    };

     let result = diesel::insert_into(bank_accounts)
         .values(&new_bank_account)
         .execute(conn)
         .expect("Error creating new transaction.");

     Ok(result)
}

pub fn pull_bank_accounts(conn: &mut PgConnection) -> Result<Vec<BankAccount>,AppError> {

     let results = bank_accounts
         .load::<BankAccount>(conn)
         .expect("Error loading transactions.");

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
