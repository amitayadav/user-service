extern crate actix_web;
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate listenfd;
extern crate user_service;
extern crate uuid;

use actix_web::{App, http, server};
use index::*;
use listenfd::ListenFd;

pub mod index;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/add/user", |r| r.method(http::Method::POST)
                .with(create_user1))
           /* .resource("/show/user/{id}", |r| r.method(http::Method::GET)
                .with(get_user))
            .resource("/show/users", |r| r.method(http::Method::GET)
                .with(get_users))*/
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3500").unwrap()
    };

    server.run();
}
