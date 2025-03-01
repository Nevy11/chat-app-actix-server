// @generated automatically by Diesel CLI.

diesel::table! {
    chat_users_table (username) {
        username -> Varchar,
        userpassword -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    mental_users_init (useremail) {
        useremail -> Varchar,
        username -> Varchar,
        userpassword -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chat_users_table,
    mental_users_init,
);
