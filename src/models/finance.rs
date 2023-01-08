use serde::{Deserialize,Serialize};
use crate::schema::{finance_incexp};

#[derive(Serialize,Deserialize,Queryable)]
pub struct FinanceIncExp {
    pub id: i32,
    pub person_id: i32,
    pub label: String,
    pub item_link: String,
    pub amount: f32,
    pub category_id: i32,
    pub currency_id: i16,
    pub transaction_type_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = finance_incexp)]
pub struct NewFinanceIncExp<'a> {
    pub person_id: i32,
    pub label: &'a str,
    pub item_link: &'a str,
    pub amount: f32,
    pub category_id: i32,
    pub currency_id: i16,
    pub transaction_type_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFinanceIncExpHandler {
    pub label: String,
    pub item_link: String,
    pub amount: f32,
    pub category_id: i32,
    pub currency_id: i16,
    pub transaction_type_id: i16,
}
