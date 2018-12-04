use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::{self,types::prelude::*};
use cdrs::types::from_cdrs::FromCDRSByName;
//use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromUDT)]
pub struct User {
   pub id: i32,
   pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue)]
pub struct CreateUser {
   pub name: String
}

/*#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct UserCreated {
    data: User
}*/


pub enum UserEvent {
    UserCreated(User),
}

impl UserEvent {
   pub fn created(u_id:i32,u_name:String) -> UserEvent {
        UserEvent::UserCreated(User {
            id: u_id,
            name: u_name,
        })
    }
}

trait Aggregate {
    type Person;

    fn version(&self) -> u64;
    fn apply(&self, evt: &Self::Person) -> Self where Self:Sized;
}

#[derive(Debug)]
pub struct UserAggregate {
    version: u64,
    id: i32,
    name: String,
}

impl UserAggregate {
   pub fn create_user(user_id:i32,user_name:String)-> Result<Vec<UserEvent>> {
       UserAggregate{
           version:1,
           id:user_id,
           name:user_name.to_owned(),
       };
       Ok(vec![UserEvent::created(user_id,user_name)])
    }
}

impl Aggregate for UserAggregate {
    type Person = UserEvent;

    fn version(&self) -> u64 {
        self.version
    }
    fn apply(&self, evt: &UserEvent) -> UserAggregate {
        UserAggregate {
            version: self.version + 1,
            id: 101,
            name: match evt {
                &UserEvent::UserCreated(User { ref name, .. }) => name,
                _ => &self.name
            }.parse().unwrap(),
        }
    }
}