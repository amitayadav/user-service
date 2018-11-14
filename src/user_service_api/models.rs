use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::{self,types::prelude::*};
use cdrs::types::from_cdrs::FromCDRSByName;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct User {
   pub id: String,
   pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue)]
pub struct CreatedUser {
   pub name: String
}
