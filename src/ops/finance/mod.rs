use diesel::prelude::*;

use crate::models::finance::{NewFinanceIncExp,FinanceIncExp,InputFinanceIncExpHandler};
use crate::schema::finance_incexp::dsl::*;
use super::DbError;

pub fn push_incexp(conn: &mut PgConnection, input: crate::web::Json<InputFinanceIncExpHandler>) -> Result<usize,DbError> {

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
         //.get_result(conn)?;
         .execute(conn)
         .expect("Error creating new income/expense item.");

     Ok(result)
}

pub fn pull_incexps(conn: &mut PgConnection) -> Result<Vec<FinanceIncExp>,DbError> {
    
     let results = finance_incexp
         .load::<FinanceIncExp>(conn)
         .expect("Error loading income/expenditure list.");
         //.optional()?;

     Ok(results)
}
