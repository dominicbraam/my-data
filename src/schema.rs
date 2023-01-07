// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        dob -> Date,
    }
}
