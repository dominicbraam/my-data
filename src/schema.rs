// @generated automatically by Diesel CLI.

diesel::table! {
    bank_account_balance_history (id) {
        id -> Int4,
        account_id -> Int4,
        balance -> Numeric,
        record_date -> Timestamp,
    }
}

diesel::table! {
    bank_account_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

diesel::table! {
    bank_accounts (id) {
        id -> Int4,
        person_id -> Int4,
        account_type_id -> Int4,
        currency_id -> Int4,
        balance -> Numeric,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int4,
        code -> Bpchar,
        name -> Varchar,
    }
}

diesel::table! {
    document_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

diesel::table! {
    documents (id) {
        id -> Int4,
        person_id -> Int4,
        document_type_id -> Int4,
        file_path -> Text,
        description -> Nullable<Text>,
        uploaded_at -> Timestamp,
    }
}

diesel::table! {
    emails (id) {
        id -> Int4,
        person_id -> Int4,
        email -> Varchar,
    }
}

diesel::table! {
    persons (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        dob -> Date,
        password_hash -> Varchar,
    }
}

diesel::table! {
    transaction_actions (id) {
        id -> Int4,
        actions -> Varchar,
        transaction_type_id -> Int4,
    }
}

diesel::table! {
    transaction_groups (id) {
        id -> Int4,
        description -> Text,
    }
}

diesel::table! {
    transaction_products (id) {
        id -> Int4,
        name -> Varchar,
        product_link -> Nullable<Text>,
        description -> Nullable<Text>,
        price -> Numeric,
        currency_id -> Int4,
    }
}

diesel::table! {
    transaction_tags (id) {
        id -> Int4,
        tag -> Varchar,
    }
}

diesel::table! {
    transaction_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        group_id -> Nullable<Int4>,
        account_id -> Int4,
        action_id -> Int4,
        tag_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        document_id -> Nullable<Int4>,
        amount -> Numeric,
        transaction_date -> Date,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    wishlist (id) {
        id -> Int4,
        person_id -> Int4,
        product_id -> Nullable<Int4>,
        added_date -> Date,
        list_order -> Int4,
    }
}

diesel::joinable!(bank_account_balance_history -> bank_accounts (account_id));
diesel::joinable!(bank_accounts -> bank_account_types (account_type_id));
diesel::joinable!(bank_accounts -> currencies (currency_id));
diesel::joinable!(bank_accounts -> persons (person_id));
diesel::joinable!(documents -> document_types (document_type_id));
diesel::joinable!(documents -> persons (person_id));
diesel::joinable!(emails -> persons (person_id));
diesel::joinable!(transaction_actions -> transaction_types (transaction_type_id));
diesel::joinable!(transaction_products -> currencies (currency_id));
diesel::joinable!(transactions -> bank_accounts (account_id));
diesel::joinable!(transactions -> documents (document_id));
diesel::joinable!(transactions -> transaction_actions (action_id));
diesel::joinable!(transactions -> transaction_groups (group_id));
diesel::joinable!(transactions -> transaction_products (product_id));
diesel::joinable!(transactions -> transaction_tags (tag_id));
diesel::joinable!(wishlist -> persons (person_id));
diesel::joinable!(wishlist -> transaction_products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    bank_account_balance_history,
    bank_account_types,
    bank_accounts,
    currencies,
    document_types,
    documents,
    emails,
    persons,
    transaction_actions,
    transaction_groups,
    transaction_products,
    transaction_tags,
    transaction_types,
    transactions,
    wishlist,
);
