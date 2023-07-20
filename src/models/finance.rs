use serde::{Deserialize,Serialize};
use crate::schema::{transactions,bank_accounts};
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

#[derive(Serialize,Deserialize,Queryable)]
pub struct BankAccount {
    pub id: i32,
    pub person_id: i32,
    pub account_type_id: i32,
    pub currency_id: i32,
    pub balance: BigDecimal,
    pub branch_id: i32,
    pub account_number: String,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = bank_accounts)]
pub struct NewBankAccount {
    pub person_id: i32,
    pub account_type_id: i32,
    pub currency_id: i32,
    pub balance: BigDecimal,
    pub branch_id: i32,
    pub account_number: String,
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct BankBranch {
    pub id: i32,
    pub bank_id: i32,
    pub name: String,
    pub street: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country_id: i32,
    pub swift: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct Currency {
    pub id: i32,
    pub code: String,
    pub name: String,
}
