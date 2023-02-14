// @generated automatically by Diesel CLI.

diesel::table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        order_number -> Integer,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    requests (id) {
        id -> Integer,
        name -> Text,
        url -> Nullable<Text>,
        method -> Nullable<Text>,
        order_number -> Integer,
        parent_id -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    folders,
    requests,
);
