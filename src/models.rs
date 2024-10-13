use diesel::{
    pg::Pg,
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Selectable, Queryable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::chat_users_table)]
#[diesel(check_for_backend(Pg))]
pub struct ChatUsers {
    pub username: String,
    pub userpassword: String,
    pub email: String,
}

#[derive(Insertable, Selectable, Queryable, Deserialize)]
#[diesel(table_name = crate::schema::chat_users_table)]
#[diesel(check_for_backend(Pg))]
pub struct LoginChatUsers {
    pub username: String,
    pub userpassword: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPassword {
    pub username: String,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}
