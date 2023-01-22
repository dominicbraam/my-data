use serde::{Deserialize,Serialize};
use crate::schema::{finance_transaction};

#[derive(Serialize,Deserialize,Queryable)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub description: String,
    pub bank_description: Option<String>,
    pub item_link: Option<String>,
    pub amount: f32,
    pub tentative_amount: Option<f32>,
    pub is_amount_tentative: bool,
    pub category_id: i32,
    pub currency_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = finance_transaction)]
pub struct NewTransaction<'a> {
    pub account_id: i32,
    pub description: &'a str,
    pub bank_description: Option<&'a str>,
    pub item_link: Option<&'a str>,
    pub amount: f32,
    pub tentative_amount: Option<f32>,
    pub is_amount_tentative: bool,
    pub category_id: i32,
    pub currency_id: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTransactionHandler {
    pub description: String,
    pub bank_description: Option<String>,
    pub item_link: Option<String>,
    pub amount: f32,
    pub tentative_amount: Option<f32>,
    pub is_amount_tentative: bool,
    pub category_id: i32,
    pub currency_id: i16,
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct Currency {
    pub id: i16,
    pub label: String,
    pub abbreviation: String,
}
