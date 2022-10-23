// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        dob -> Date,
    }
}
