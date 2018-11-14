use actix_web::{Json, Path, Result};
use std::collections::*;
use user_service::set_up_databse::{connection::*,keyspace::*,table::*};
use user_service::user_service_api::{models::*,user_service_impl::*};

pub fn create_user(createduser: Json<CreatedUser>) -> Result<Json<User>> {
    let session = connect();
    create_keyspace(&session);
    create_table(&session);
    insert_struct(&session, createduser)
}

pub fn get_user(path: Path<String>) -> Result<Json<User>> {
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
    match select_struct(&connect()) {
        Vec[User{id,name},User{id,name},...] =>
        Ok(Vec[User{id,name},User{id,name},...]),
        _ => Err("there is some problem in fetching all users")
    }
}


