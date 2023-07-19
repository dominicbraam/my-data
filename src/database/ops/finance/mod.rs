use diesel::prelude::*;

use crate::models::finance::{NewTransaction,Transaction,InputTransactionHandler,Currency};
use crate::schema::transactions::dsl::*;
use crate::schema::currencies::dsl::*;
use crate::error::AppError;

pub fn push_incexp(conn: &mut PgConnection, input: crate::web::Json<InputTransactionHandler>) -> Result<usize,AppError> {

    let new_transaction = NewTransaction {
        group_id: Some(1),
        account_id: 1,
        action_id: 1,
        tag_id: Some(1),
        product_id: Some(1),
        document_id: Some(1),
        is_need: Some(true),
        amount: input.amount.clone(),
        transaction_date: chrono::Utc::now().date_naive(),
        description: input.description.clone(),
       /*  description: match &input.description {
            Some(_x) => {
                    let desc = &input.description;
                    Some(desc.as_ref().unwrap())
            },
            None => None
        },
  */   };

     let result = diesel::insert_into(transactions)
         .values(&new_transaction)
         .execute(conn)
         .expect("Error creating new transaction.");

     Ok(result)
}

pub fn pull_incexps(conn: &mut PgConnection) -> Result<Vec<Transaction>,AppError> {
    
     let results = transactions
         .load::<Transaction>(conn)
         .expect("Error loading transactions.");

     Ok(results)
}

pub fn pull_currencies(conn: &mut PgConnection) -> Result<Vec<Currency>,AppError> {
    
     let results = currencies
         .load::<Currency>(conn)
         .expect("Error loading currency list.");

     Ok(results)
}
