use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::{self,types::prelude::*};
use cdrs::types::from_cdrs::FromCDRSByName;
//use uuid::Uuid;
use cdrs::types::value::{Value, Bytes};
//use cdrs::IntoCDRSValue;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromUDT)]
pub struct User {
   pub id: i32,
   pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue)]
pub struct CreateUser {
   pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue,TryFromUDT)]
pub struct UserCreated {
    pub user: User,
    pub event_type: String,
}

/*#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue,TryFromUDT)]
pub struct UserUpdated {
    pub user: User,
    pub event_type: String,
}*/

#[derive(Debug, IntoCDRSValue,TryFromRow)]
pub struct UserEvent {
    pub id:i32,
    pub user_create: UserCreated,
}


impl UserEvent {
   pub fn created(user:User) -> UserEvent {
         UserEvent {  id:user.id.to_owned(),
                      user_create: UserCreated {
                          user: User {
                              id: user.id,
                              name: user.name,
                          },
                          event_type: "UserCreated".to_string(),
                      }
       }
   }
}

pub trait Aggregate {
    type Person;

    fn version(&self) -> u64;
    fn apply(mut self, evt: &Self::Person) -> Self where Self:Sized;
}

#[derive(Debug)]
pub struct UserAggregate {
    version: u64,
    id: i32,
    name: String,
}

impl UserAggregate {
   pub fn create_user(user_data:User)-> UserAggregate {
       UserAggregate{
           version:1,
           id:user_data.id.to_owned(),
           name:user_data.name.to_owned(),
       }
       //Ok(vec![UserEvent::created(user_id,user_name)])
    }
}

impl Aggregate for UserAggregate {
    type Person = UserEvent;

    fn version(&self) -> u64 {
        self.version
    }
    fn apply(mut self, evt: &UserEvent) -> UserAggregate {
       // let evt1:&UserEvent=evt.to_owned();
        self.version += 1;
        self.id = evt.id;
        let n=&evt.user_create;
        self.name = n.to_owned().user.name;
        self
    }
}


