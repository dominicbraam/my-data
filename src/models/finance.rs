use serde::{Deserialize,Serialize};
use crate::schema::finance;

#[derive(Serialize,Deserialize,Queryable)]
pub struct Finance_IncExp {
    pub id: i32,
    pub person_id: i32,
    pub label: String,
    pub item_link: String,
    pub amount: f32,
    pub currency_id: i16,
    pub transaction_type: i16,
    pub created_date: chrono::NaiveDate,
    pub updated_date: chrono::NaiveDate,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = finance_incexp)]
pub struct NewFinance_IncExp<'a> {
    pub label: &'a str,
    pub item_link: &'a str,
    pub amount: &'a f32,
    pub currency_id: &'a i16,
    pub transaction_type: &'a i16,
    pub created_date: chrono::NaiveDate,
    pub updated_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFinanceIncExp {
    pub label: &'a str,
    pub item_link: &'a str,
    pub amount: &'a f32,
    pub currency_id: &'a i16,
    pub transaction_type: &'a i16,
}
