// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        is_admin -> Bool,
    }
}
