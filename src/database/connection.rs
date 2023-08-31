use std::env;
use diesel::pf::PgConnection;

// pub fn make_connection() -> Option<String> {
pub fn make_connection() {
    let db_url = env::var("DATABASE_URL").expect("db url not found");
    let connection = PgConnection::establish(&db_url);
}