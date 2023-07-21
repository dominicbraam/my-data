// @generated automatically by Diesel CLI.

diesel::table! {
    address_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

diesel::table! {
    addresses (id) {
        id -> Int4,
        person_id -> Nullable<Int4>,
        address_type_id -> Nullable<Int4>,
        street -> Varchar,
        city -> Varchar,
        state -> Nullable<Varchar>,
        country_id -> Nullable<Int4>,
        postal_code -> Nullable<Varchar>,
        is_legal -> Bool,
        is_billing -> Bool,
        is_shipping -> Bool,
        description -> Nullable<Text>,
        archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    bank_account_balance_history (id) {
        id -> Int4,
        account_id -> Int4,
        balance -> Numeric,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        transaction_id -> Int4,
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
        archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        branch_id -> Int4,
        account_number -> Varchar,
    }
}

diesel::table! {
    bank_branches (id) {
        id -> Int4,
        record_group -> Int4,
        is_current -> Bool,
        bank_id -> Int4,
        name -> Varchar,
        street -> Nullable<Text>,
        city -> Text,
        state -> Nullable<Varchar>,
        postal_code -> Nullable<Varchar>,
        country_id -> Int4,
        swift -> Nullable<Varchar>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    banks (id) {
        id -> Int4,
        record_group -> Int4,
        is_current -> Bool,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        record_group -> Int4,
        is_current -> Bool,
        name -> Varchar,
        code -> Varchar,
        currency_main_id -> Int4,
        created_at -> Timestamp,
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
        record_group -> Int4,
        is_current -> Bool,
        person_id -> Int4,
        document_type_id -> Int4,
        file_path -> Text,
        description -> Nullable<Text>,
        archived -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    emails (id) {
        id -> Int4,
        person_id -> Int4,
        email -> Varchar,
        archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
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
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transaction_products (id) {
        id -> Int4,
        record_group -> Int4,
        is_current -> Bool,
        name -> Varchar,
        product_link -> Nullable<Text>,
        description -> Nullable<Text>,
        price -> Numeric,
        currency_id -> Int4,
        archived -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    transaction_tags (id) {
        id -> Int4,
        person_id -> Int4,
        tag -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
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
        is_need -> Nullable<Bool>,
        amount -> Numeric,
        transaction_datetime -> Timestamp,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    wishlist (id) {
        id -> Int4,
        person_id -> Int4,
        product_id -> Nullable<Int4>,
        list_order -> Int4,
        archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(addresses -> address_types (address_type_id));
diesel::joinable!(addresses -> countries (country_id));
diesel::joinable!(addresses -> persons (person_id));
diesel::joinable!(bank_account_balance_history -> bank_accounts (account_id));
diesel::joinable!(bank_account_balance_history -> transactions (transaction_id));
diesel::joinable!(bank_accounts -> bank_account_types (account_type_id));
diesel::joinable!(bank_accounts -> bank_branches (branch_id));
diesel::joinable!(bank_accounts -> currencies (currency_id));
diesel::joinable!(bank_accounts -> persons (person_id));
diesel::joinable!(bank_branches -> banks (bank_id));
diesel::joinable!(bank_branches -> countries (country_id));
diesel::joinable!(countries -> currencies (currency_main_id));
diesel::joinable!(documents -> document_types (document_type_id));
diesel::joinable!(documents -> persons (person_id));
diesel::joinable!(emails -> persons (person_id));
diesel::joinable!(transaction_actions -> transaction_types (transaction_type_id));
diesel::joinable!(transaction_products -> currencies (currency_id));
diesel::joinable!(transaction_tags -> persons (person_id));
diesel::joinable!(transactions -> bank_accounts (account_id));
diesel::joinable!(transactions -> documents (document_id));
diesel::joinable!(transactions -> transaction_actions (action_id));
diesel::joinable!(transactions -> transaction_groups (group_id));
diesel::joinable!(transactions -> transaction_products (product_id));
diesel::joinable!(transactions -> transaction_tags (tag_id));
diesel::joinable!(wishlist -> persons (person_id));
diesel::joinable!(wishlist -> transaction_products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    address_types,
    addresses,
    bank_account_balance_history,
    bank_account_types,
    bank_accounts,
    bank_branches,
    banks,
    countries,
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
