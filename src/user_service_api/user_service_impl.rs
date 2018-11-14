use set_up_databse::connection::*;
use user_service_api::models::*;
use actix_web::{Json,Path};
use cdrs::types::prelude::*;
use cdrs::query::QueryExecutor;

pub fn insert_struct(session: &CurrentSession, new_user: Json<CreatedUser>) -> Json<User> {

    let user = User {
        id: "101".to_string(),
        name: new_user.name.to_string()
    };
    let insert_struct_cql = "INSERT INTO user_ks.user \
                           (id,name) VALUES (?,?) ";
    session
        .query_with_values(insert_struct_cql, query_values!(user.id,user.name))
        .expect("insert here ");

    Json(user)
}

pub fn select_one_struct(session: &CurrentSession, path: Path<String>) -> User{
    let select_struct_cql = "SELECT * FROM user_ks.user where id = ?";
    let id = path.into_inner();

    let rows = session.query_with_values(select_struct_cql, query_values!(id))
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let mut my_row = User {
        id: String::new(),
        name: String::new(),
    };

    for row in rows {
        my_row = User::try_from_row(row).expect("into User")
    }

    my_row
}

pub fn select_struct(session: &CurrentSession) -> Vec<User>{
    let select_struct_cql = "SELECT * FROM user_ks.user ";

    let rows = session.query(select_struct_cql)
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let mut v: Vec<User> = Vec::new();
    for row in rows {
         v.push(User::try_from_row(row).expect("into User"))
    }
    v
}