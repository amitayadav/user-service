use set_up_databse::connection::*;
use cdrs::query::QueryExecutor;

pub fn create_user_udt(session: &CurrentSession) {
    let create_type_cql =
        "CREATE TYPE IF NOT EXISTS user_ks.user (id int,name text);";
    session
        .query(create_type_cql)
        .expect("user Type creation error");
}
/*
pub fn create_event_udt(session: &CurrentSession) {
    let create_type_cql =
        "CREATE TYPE IF NOT EXISTS user_ks.event (data frozen<user>);";
    session
        .query(create_type_cql)
        .expect("event Type creation error");
}*/

pub fn create_event_table(session: &CurrentSession) {
    let create_table_cql = "CREATE TABLE IF NOT EXISTS user_ks.user_event (id int PRIMARY KEY,variant text)";
    session
        .query(create_table_cql)
        .expect("event Table creation error");
}