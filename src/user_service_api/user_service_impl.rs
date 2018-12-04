use set_up_databse::connection::*;
use user_service_api::models::*;
use actix_web::{Json,Path};
use cdrs::types::prelude::*;
use cdrs::query::QueryExecutor;
//use uuid::*;

pub fn insert_struct(session: &CurrentSession, new_user: Json<CreateUser>)  {

   let userId = 101;

    let user_data = User {
        id: userId.to_owned(),
        name: new_user.name.to_string()
    };
    let event = match UserAggregate::create_user(userId,user_data.name)
        {
        vec![UserEvent::UserCreated,..] => vec![UserEvent::UserCreated(user_data),..]
            .iter().map(|x|x).collect(),
        _ => "not created".to_string(),
    };

    let insert_struct_cql = "INSERT INTO user_ks.user_event \
                           (id,variant) VALUES (?,?) ";
    session
        .query_with_values(insert_struct_cql, query_values!(userId,event))
        .expect("insert here ");
        //Json(user_data)

}

/*
pub fn select_one_struct(session: &CurrentSession, path: Path<i32>) -> User{
    let select_struct_cql = "SELECT * FROM user_ks.user where id = ?";
    let id = path.into_inner();

    let rows = session.query_with_values(select_struct_cql, query_values!(id))
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let mut my_row = User {
        id: 0,
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
       // v = User::try_from_row(row).expect("into User");
    }
    v
}*/
