use diesel::prelude::*;

use crate::{
    chat_connectivity::connectivity_establishment::establish_connection,
    models::ChatUsers,
    schema::chat_users_table::{self, username},
};

/// This function reads one data of the chat and returns a result of the chat users
/// struct.
pub fn read_one_chat_user(name_of_user: String) -> Result<ChatUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    chat_users_table::dsl::chat_users_table
        .filter(username.eq(name_of_user.to_uppercase()))
        .select(ChatUsers::as_returning())
        .get_result(connection)
}

/// This function returns all chat users that are stored in the database together with
/// their passwords
pub fn read_all_chat_user() -> Result<Vec<ChatUsers>, diesel::result::Error> {
    let connection = &mut establish_connection();
    chat_users_table::dsl::chat_users_table::load::<ChatUsers>(chat_users_table::table, connection)
}

/// This method takes in the user name of the user, checks to see if the user is in the
/// database, returning false if the user is not signed_up but true if matches. The password
/// is compared against that stored in the database.
pub fn check_for_users_password(name_of_user: String, password_of_user: String) -> bool {
    // check to see if the user is in the database
    let user_data_result = read_one_chat_user(name_of_user.to_uppercase());
    match user_data_result {
        Ok(user_data) => {
            if password_of_user == user_data.userpassword {
                println!("Log in successfully");
                true
            } else {
                println!("Incorrect password when loggin in");
                false
            }
        }
        Err(e) => {
            println!("the user is not in the read one chat user when checking for password");
            println!("{e:?}");
            false
        }
    }
}
