// @generated automatically by Diesel CLI.

diesel::table! {
    finance_account (id) {
        id -> Int4,
        person_id -> Int4,
        name -> Text,
        account_type_id -> Int2,
        currency_id -> Int2,
    }
}

diesel::table! {
    finance_account_balance (id) {
        id -> Int4,
        account_id -> Int4,
        amount -> Float4,
    }
}

diesel::table! {
    finance_account_type (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    finance_category (id) {
        id -> Int4,
        name -> Text,
        transaction_type_id -> Int2,
    }
}

diesel::table! {
    finance_currency (id) {
        id -> Int2,
        label -> Text,
        abbreviation -> Text,
    }
}

diesel::table! {
    finance_transaction (id) {
        id -> Int4,
        account_id -> Int4,
        description -> Text,
        bank_description -> Nullable<Text>,
        item_link -> Nullable<Text>,
        amount -> Float4,
        tentative_amount -> Nullable<Float4>,
        is_amount_tentative -> Bool,
        category_id -> Int4,
        currency_id -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    finance_transaction_type (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    person (id) {
        id -> Int4,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        dob -> Date,
    }
}

diesel::joinable!(finance_account -> finance_account_type (account_type_id));
diesel::joinable!(finance_account -> finance_currency (currency_id));
diesel::joinable!(finance_account -> person (person_id));
diesel::joinable!(finance_account_balance -> finance_account (account_id));
diesel::joinable!(finance_category -> finance_transaction_type (transaction_type_id));
diesel::joinable!(finance_transaction -> finance_account (account_id));
diesel::joinable!(finance_transaction -> finance_category (category_id));
diesel::joinable!(finance_transaction -> finance_currency (currency_id));

diesel::allow_tables_to_appear_in_same_query!(
    finance_account,
    finance_account_balance,
    finance_account_type,
    finance_category,
    finance_currency,
    finance_transaction,
    finance_transaction_type,
    person,
);
