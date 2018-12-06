extern crate actix_web;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod user_service_api {
    pub mod models;
    pub mod user_service_impl;
}

pub mod set_up_databse {
    pub mod connection;
    pub mod keyspace;
    pub mod table;
}