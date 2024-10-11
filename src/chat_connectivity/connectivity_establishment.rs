use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

/// This function establishes the connection between the database and
/// the actix web server before making any query
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}
