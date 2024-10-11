use diesel::prelude::*;

use crate::{
    chat_connectivity::connectivity_establishment::establish_connection,
    models::ChatUsers,
    schema::chat_users_table::{self, username},
};

pub fn delete_chat_user(name_of_user: String) -> Result<ChatUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(chat_users_table::dsl::chat_users_table)
        .filter(username.eq(name_of_user.to_uppercase()))
        .returning(ChatUsers::as_returning())
        .get_result(connection)
}
