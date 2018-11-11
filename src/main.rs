#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]


#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

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
extern crate oauth2;
extern crate reqwest;
extern crate select;
extern crate url;

extern crate frank_jwt;
extern crate inflector;

extern crate csv;
extern crate openssl_probe;


use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashSet;
use std::time::Instant;

use rusqlite::Connection;

mod face;
mod db;
mod email;
mod api;
mod settings;
mod logger;
mod google;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let conn = Connection::open("./face.db").unwrap();
    db::create_db(&conn);
    let personality_stats = db::get_personality_stats(&conn);
    for stat in personality_stats {
        println!("{} {}", stat.0, stat.1);
    }
    let config = Arc::new(Mutex::new(api::BackendState {
        db_conn: conn,
        seen_faces: HashSet::new(),
        last_face_time: Instant::now(),
    }));

    logger::set_logging();

    // email::send_email("maksim.levental@gmail.com", "maks", "neurotic");
    // email::send_email("lh00000000@gmail.com", "lh", "neurotic");
    // email::send_old_emails();

    api::start_api(config);
}
