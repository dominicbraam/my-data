use serde::{
    Serialize,
    Deserialize,
};
use diesel::{
    Insertable,
    Queryable,
};
use crate::schema::financial;
use bigdecimal::BigDecimal;

#[derive(Serialize,Deserialize,Queryable)]
#[diesel(table_name = financial::transactions)]
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
    pub transaction_datetime: chrono::NaiveDateTime,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = financial::transactions)]
pub struct NewTransaction {
    pub group_id: Option<i32>,
    pub account_id: i32,
    pub action_id: i32,
    pub tag_id: Option<i32>,
    pub product_id: Option<i32>,
    pub document_id: Option<i32>,
    pub is_need: Option<bool>,
    pub amount: BigDecimal,
    pub transaction_datetime: chrono::NaiveDateTime,
    pub description: Option<String>,
}

#[derive(Serialize,Deserialize,Queryable)]
#[diesel(table_name = financial::bank_accounts)]
pub struct BankAccount {
    pub id: i32,
    pub person_id: i32,
    pub account_type_id: i32,
    pub currency_id: i32,
    pub archived: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub branch_id: i32,
    pub account_number: String,
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = financial::bank_accounts)]
pub struct NewBankAccount {
    pub person_id: i32,
    pub account_type_id: i32,
    pub currency_id: i32,
    pub branch_id: i32,
    pub account_number: String,
}

#[derive(Serialize,Deserialize,Queryable)]
#[diesel(table_name = financial::bank_brances)]
pub struct BankBranch {
    pub id: i32,
    pub record_group: i32,
    pub is_current: bool,
    pub bank_id: i32,
    pub name: String,
    pub street: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country_id: i32,
    pub swift: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize,Queryable)]
#[diesel(table_name = financial::currencies)]
pub struct Currency {
    pub id: i32,
    pub code: String,
    pub name: String,
}
