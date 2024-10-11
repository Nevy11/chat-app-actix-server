use diesel::{
    pg::Pg,
    prelude::{Insertable, Queryable},
    Selectable,
};

#[derive(Insertable, Selectable, Queryable)]
#[diesel(table_name = crate::schema::chat_users_table)]
#[diesel(check_for_backend(Pg))]
pub struct ChatUsers {
    pub username: String,
    pub userpassword: String,
}
