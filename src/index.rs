use actix_web::{Json, Path, Result};
use std::collections::*;
use user_service::set_up_databse::{connection::*,keyspace::*,table::*};
use user_service::user_service_api::{models::*,user_service_impl::*};
//use uuid::Uuid;

pub fn create_user1(create: Json<CreateUser>) -> Json<User> {
    let session = connect();
    create_keyspace(&session);
    create_user_udt(&session);
    create_event_udt(&session);
    create_event_table(&session);
    insert_struct(&session, create)
   // Ok("event stored successfully".to_string())

}

/*
pub fn get_user(path: Path<i32>) -> Json<UserEvent>{
    let user_event = select_one_struct(&connect(), path);

    match user_event {
        UserEvent{id,user_create}=> Json(user_event),
        _ => Json(None),
    }

}

pub fn get_users() -> Vec<UserEvent> {
    let user_list = select_struct(&connect());
    match user_list.iter() {
        UserEvent{id,user_create} =>
        vec![UserEvent{id,user_create}],
        _ => vec![]
    }
}
*/
