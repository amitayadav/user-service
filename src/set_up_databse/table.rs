use set_up_databse::connection::*;
use cdrs::query::QueryExecutor;

pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS user_ks.user (id text PRIMARY KEY , \
     name text);";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}