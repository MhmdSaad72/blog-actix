// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}
