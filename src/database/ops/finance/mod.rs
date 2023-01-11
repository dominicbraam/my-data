use diesel::prelude::*;

use crate::models::finance::{NewFinanceIncExp,FinanceIncExp,InputFinanceIncExpHandler,FinanceCurrency};
use crate::schema::finance_incexp::dsl::*;
use crate::schema::finance_currency::dsl::*;
use crate::error::AppError;

pub fn push_incexp(conn: &mut PgConnection, input: crate::web::Json<InputFinanceIncExpHandler>) -> Result<usize,AppError> {

    let new_finance_incexp = NewFinanceIncExp {
        person_id: 1,
        label: &input.label,
        item_link: &input.item_link,
        amount: input.amount,
        currency_id: input.currency_id,
        category_id: input.category_id,
        transaction_type_id: input.transaction_type_id,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };

     let result = diesel::insert_into(finance_incexp)
         .values(&new_finance_incexp)
         .execute(conn)
         .expect("Error creating new income/expense item.");

     Ok(result)
}

pub fn pull_incexps(conn: &mut PgConnection) -> Result<Vec<FinanceIncExp>,AppError> {
    
     let results = finance_incexp
         .load::<FinanceIncExp>(conn)
         .expect("Error loading income/expenditure list.");

     Ok(results)
}

pub fn pull_currencies(conn: &mut PgConnection) -> Result<Vec<FinanceCurrency>,AppError> {
    
     let results = finance_currency
         .load::<FinanceCurrency>(conn)
         .expect("Error loading currency list.");

     Ok(results)
}
