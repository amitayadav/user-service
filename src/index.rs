use actix_web::{Json, Path, Result};
use std::collections::*;
use user_service::set_up_databse::{connection::*,keyspace::*,table::*};
use user_service::user_service_api::{models::*,user_service_impl::*};
//use uuid::Uuid;

pub fn create_user1(create: Json<CreateUser>) -> Result<String> {
    let session = connect();
    create_keyspace(&session);
    create_user_udt(&session);
    //create_event_udt(&session);
    create_event_table(&session);
    insert_struct(&session, create);
    Ok("event stored successfully".to_string())

}

/*
pub fn get_user(path: Path<i32>) -> Result<Json<User>> {
    let user = select_one_struct(&connect(), path);

    match user {
        User {id,name} => Ok(Json(User{
            id: user.id,
            name:user.name
        })),
        _ => Err("user does not found with this id")
    }

}

pub fn get_users() -> Result<Vec<User>> {
    let user_list = select_struct(&connect());
    match user_list.iter() {
        [User{id,name},rest] =>
        Ok(vec![User{id,name},rest]),
        _ => Err("there is some problem in fetching all users")
    }
}


*/
