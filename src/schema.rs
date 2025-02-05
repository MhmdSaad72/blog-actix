// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
