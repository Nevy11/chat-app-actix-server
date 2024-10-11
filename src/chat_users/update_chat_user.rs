use diesel::prelude::*;

use crate::{
    chat_connectivity::connectivity_establishment::establish_connection,
    models::ChatUsers,
    schema::chat_users_table::{self, username, userpassword},
};

/// If it returns None, it means that the user has entered an invalid field.
/// else it returns an option of the result of ChatUsers struct.
pub fn update_chat_user(
    name_of_user: String,
    new_value: String,
    field: String,
) -> Option<Result<ChatUsers, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let value_of_field = field.as_str();
    match value_of_field {
        "USERNAME" => Some(
            diesel::update(chat_users_table::dsl::chat_users_table)
                .filter(username.eq(name_of_user.to_uppercase()))
                .set(username.eq(new_value.to_uppercase()))
                .returning(ChatUsers::as_returning())
                .get_result(connection),
        ),
        "USERPASSWORD" => Some(
            diesel::update(chat_users_table::dsl::chat_users_table)
                .filter(username.eq(name_of_user.to_uppercase()))
                .set(userpassword.eq(new_value))
                .returning(ChatUsers::as_returning())
                .get_result(connection),
        ),
        _ => None,
    }
}
