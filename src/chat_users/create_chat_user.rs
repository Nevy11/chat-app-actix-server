use crate::chat_connectivity::connectivity_establishment::establish_connection;
#[allow(dead_code)]
pub fn create_chat_user() -> Option<String> {
    #[allow(unused_variables)]
    let connection = &mut establish_connection();
    Some(String::from("Still developing"))
}
