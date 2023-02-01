// @generated automatically by Diesel CLI.

diesel::table! {
    request (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        method -> Text,
    }
}
