// @generated automatically by Diesel CLI.

diesel::table! {
    finance_category (id) {
        id -> Int4,
        name -> Text,
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
    finance_incexp (id) {
        id -> Int4,
        person_id -> Int4,
        label -> Text,
        item_link -> Text,
        amount -> Float4,
        category_id -> Int4,
        currency_id -> Int2,
        transaction_type_id -> Int2,
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
    finance_worthstat (id) {
        id -> Int4,
        person_id -> Int4,
        amount -> Float4,
        currency_id -> Int2,
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

diesel::joinable!(finance_incexp -> finance_category (category_id));
diesel::joinable!(finance_incexp -> finance_currency (currency_id));
diesel::joinable!(finance_incexp -> finance_transaction_type (transaction_type_id));
diesel::joinable!(finance_incexp -> person (person_id));
diesel::joinable!(finance_worthstat -> finance_currency (currency_id));
diesel::joinable!(finance_worthstat -> person (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    finance_category,
    finance_currency,
    finance_incexp,
    finance_transaction_type,
    finance_worthstat,
    person,
);
