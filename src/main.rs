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


use rocket_contrib::{Json, Value};
use std::io;

use std::path::{Path, PathBuf};
use std::sync::{Mutex, Arc};
use std::thread;
use ws::listen;
use rocket::response::NamedFile;

use rusqlite::{Connection, Error as RusqliteError};
use rocket::State;

mod face;
mod db;


struct Config {
    db_conn: Connection,
}

//
type ID = String;
type ServerConfig = Arc<Mutex<Config>>;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/face/<id>", format = "application/json", data = "<face_json>")]
fn new(id: ID, face_json: Json<face::Face>, conf: State<ServerConfig>) -> Json<Value> {
    let db_conn = &conf.lock().expect("get conf").db_conn;
    Json(json!({ "status": "ok" }))
}

#[get("/face/<id>", format = "application/json")]
fn get(id: ID, conf: State<ServerConfig>) -> Option<Json<face::Face>> {
    let conf = conf.lock().unwrap();
//    Some(face::Face{})
    None
}


#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error-404",
        "reason": "Resource was not found."
    }))
}


fn websocket(conf: ServerConfig) {
    let conf_clone = Arc::clone(&conf);
    thread::spawn(|| {
        listen("127.0.0.1:3012", move |out| {
            let conf_clone = Arc::clone(&conf_clone);
            move |msg| {
                let db_conn = &conf_clone.lock().expect("get conf").db_conn;
                let personalities = db::get_personality_stats(db_conn);
                let serialized_personalities = serde_json::to_string(&personalities).unwrap();
                out.send(serialized_personalities)
            }
        })
    });
}

fn main() {
    let conn = Connection::open("./test.db").unwrap();
    db::create_db(&conn);
    let personality_stats = db::get_personality_stats(&conn);
    for stat in personality_stats {
        println!("{} {}", stat.0, stat.1);
    }

    let config = Arc::new(Mutex::new(Config {
        db_conn: conn,
    }));

    websocket(Arc::clone(&config));

    rocket::ignite()
//        .mount("/", routes![new, get])
        .mount("/static", routes![index, files])
        .catch(catchers![not_found])
        .manage(Arc::clone(&config));
}
