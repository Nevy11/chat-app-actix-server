// @generated automatically by Diesel CLI.

diesel::table! {
    chat_users_table (username) {
        username -> Varchar,
        userpassword -> Varchar,
    }
}
