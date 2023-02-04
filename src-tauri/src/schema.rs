// @generated automatically by Diesel CLI.

diesel::table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    requests (id) {
        id -> Integer,
        name -> Nullable<Text>,
        url -> Nullable<Text>,
        method -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    folders,
    requests,
);
