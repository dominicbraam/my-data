use serde::{Deserialize,Serialize};
use crate::schema::{transactions};
use bigdecimal::BigDecimal;

#[derive(Serialize,Deserialize,Queryable)]
pub struct Transaction {
    pub id: i32,
    pub group_id: Option<i32>,
    pub account_id: i32,
    pub action_id: i32,
    pub tag_id: Option<i32>,
    pub product_id: Option<i32>,
    pub document_id: Option<i32>,
    pub is_need: Option<bool>,
    pub amount: BigDecimal,
    pub transaction_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub group_id: Option<i32>,
    pub account_id: i32,
    pub action_id: i32,
    pub tag_id: Option<i32>,
    pub product_id: Option<i32>,
    pub document_id: Option<i32>,
    pub is_need: Option<bool>,
    pub amount: BigDecimal,
    pub transaction_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTransactionHandler {
    pub group_id: Option<i32>,
    pub account_id: i32,
    pub action_id: i32,
    pub tag_id: Option<i32>,
    pub product_id: Option<i32>,
    pub document_id: Option<i32>,
    pub is_need: Option<bool>,
    pub amount: BigDecimal,
    pub description: Option<String>,
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct Currency {
    pub id: i32,
    pub code: String,
    pub name: String,
}
