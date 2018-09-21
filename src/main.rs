#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]


#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate ws;
extern crate rusqlite;
#[macro_use]
extern crate maplit;
extern crate sendgrid;
extern crate unidecode;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate config;
#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashSet;

use rusqlite::Connection;


mod face;
mod db;
mod mail;
mod api;
mod settings;
mod logger;

fn main() {
    let conn = Connection::open("./face.db").unwrap();
    db::create_db(&conn);
    let personality_stats = db::get_personality_stats(&conn);
    for stat in personality_stats {
        println!("{} {}", stat.0, stat.1);
    }

    let config = Arc::new(Mutex::new(settings::BackendState {
        db_conn: conn,
        seen_faces: HashSet::new(),
    }));

    logger::set_logging();

    mail::send_email("maksim.levental@gmail.com", "maks", "neurotic");
}
