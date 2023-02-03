// @generated automatically by Diesel CLI.

diesel::table! {
    requests (id) {
        id -> Integer,
        name -> Nullable<Text>,
        url -> Nullable<Text>,
        method -> Text,
    }
}
