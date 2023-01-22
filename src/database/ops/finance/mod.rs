use diesel::prelude::*;

use crate::models::finance::{NewTransaction,Transaction,InputTransactionHandler,Currency};
use crate::schema::finance_transaction::dsl::*;
use crate::schema::finance_currency::dsl::*;
use crate::error::AppError;

pub fn push_incexp(conn: &mut PgConnection, input: crate::web::Json<InputTransactionHandler>) -> Result<usize,AppError> {

    let new_transaction = NewTransaction {
        account_id: 2,
        description: &input.description,
        bank_description: match &input.bank_description {
            Some(_x) => {
                    let bank_desc = &input.bank_description;
                    Some(bank_desc.as_ref().unwrap())
            },
            None => None
        },
        item_link: match &input.item_link {
            Some(_x) => {
                let link = &input.item_link;
                Some(link.as_ref().unwrap())
            },
            None => None
        },
        amount: input.amount,
        tentative_amount: match input.tentative_amount {
            Some(_x) => {
                let tent_amt = input.tentative_amount;
                Some(tent_amt.unwrap())
            },
            None => None
        },
        is_amount_tentative: input.is_amount_tentative,
        currency_id: input.currency_id,
        category_id: input.category_id,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };

     let result = diesel::insert_into(finance_transaction)
         .values(&new_transaction)
         .execute(conn)
         .expect("Error creating new transaction.");

     Ok(result)
}

pub fn pull_incexps(conn: &mut PgConnection) -> Result<Vec<Transaction>,AppError> {
    
     let results = finance_transaction
         .load::<Transaction>(conn)
         .expect("Error loading transactions.");

     Ok(results)
}

pub fn pull_currencies(conn: &mut PgConnection) -> Result<Vec<Currency>,AppError> {
    
     let results = finance_currency
         .load::<Currency>(conn)
         .expect("Error loading currency list.");

     Ok(results)
}
