// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        #[max_length = 64]
        transaction_id -> Varchar,
        #[max_length = 64]
        sender_id -> Varchar,
        #[max_length = 64]
        recipient_id -> Varchar,
        amount_in_rs -> Int8,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        #[max_length = 32]
        status -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        balance_in_rs -> Int8,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    transactions,
    users,
);
