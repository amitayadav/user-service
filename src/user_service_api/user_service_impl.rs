use actix_web::{Json, Path};
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::*;
use set_up_databse::connection::*;
use user_service_api::models::*;
//use uuid::*;

pub fn insert_struct(session: &CurrentSession, new_user: Json<CreateUser>) -> Json<User> {
    let user_id = 101;

    let user_data = User {
        id: user_id.to_owned(),
        name: new_user.name.to_string(),
    };
    let event_data = UserAggregate::create_user(user_data.to_owned());
    let a = UserEvent::created(user_data.clone());
    let c:UserEvent = a.into();
    let _u = event_data.apply(&c);
    let b = c.user_create;

    let insert_struct_cql = "INSERT INTO user_ks.user_event \
                           (id,event) VALUES (?,?) ";
    session
        .query_with_values(insert_struct_cql, query_values!(user_id,b))
        .expect("insert here ");

    Json(user_data)
}

/*
pub fn select_one_struct(session: &CurrentSession, path: Path<i32>) -> UserEvent{
    let select_struct_cql = "SELECT * FROM user_ks.user_event where id = ?";
    let id = path.into_inner();

    let rows = session.query_with_values(select_struct_cql, query_values!(id))
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");
    let b = User{id:0,name:String::new(),};
    let a= UserCreated{data:b,};
    let mut my_row = UserEvent {
        id:0,user_create:a,
    };

    for row in rows {
        my_row = UserEvent::try_from_row(row).expect("into User")
    }

    my_row
}

pub fn select_struct(session: &CurrentSession) -> Vec<UserEvent>{
    let select_struct_cql = "SELECT * FROM user_ks.user ";

    let rows = session.query(select_struct_cql)
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    let mut v: Vec<UserEvent> = Vec::new();
    for row in rows {
         v.push(UserEvent::try_from_row(row).expect("into UserEvent"))
       // v = User::try_from_row(row).expect("into User");
    }
    v
}
*/
