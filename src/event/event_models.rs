/*
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::{self,types::prelude::*};
use cdrs::types::from_cdrs::FromCDRSByName;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct UserData {
    pub name: String
}

pub enum UserEvent {
    UserCreated()
}

impl UserEvent {
    fn created(user_name: String) -> UserEvent {
        UserEvent::UserCreated(UserData {
            name: user_name,
        })
    }
}

*//*#[derive(Debug)]
struct UserAggregate {
    user_number: usize,
    name: String,
}

impl UserAggregate {
    fn new(name:String, initial_no_of_user:usize)-> UserAggregate {
        UserAggregate {
            user_number: 1,
            name:name,
        }
    }

    fn create_user(name:String)-> Result<Vec<UserEvent>> {
      Ok(vec![UserEvent::created(name)])_
    }
}*//*
*/
