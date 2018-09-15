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
use std::collections::HashSet;

mod face;
mod db;


struct Config {
    db_conn: Connection,
    seen_faces: HashSet<face::FaceId>,
}

//
type ServerConfig = Arc<Mutex<Config>>;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/face", format = "application/json", data = "<face_json>")]
fn new(face_json: Json<face::Face>, conf: State<ServerConfig>) -> Json<Value> {
    let conf = &mut conf.lock().expect("get conf");
    let face = face_json.into_inner();
    let face_id = face.face_id.clone();
    if !conf.seen_faces.contains(&face.face_id) {
        match db::insert_face(&conf.db_conn, face) {
            true => { conf.seen_faces.insert(face_id); }
            false => {}
        }
    }
    Json(json!({ "status": "ok" }))
}

#[get("/face/<id>", format = "application/json")]
fn get(id: face::FaceId, conf: State<ServerConfig>) -> Option<Json<db::FaceEvent>> {
    let db_conn = &conf.lock().expect("get conf").db_conn;
    Some(Json(db::get_face(db_conn, id)))
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
    let conn = Connection::open("./face.db").unwrap();
    db::create_db(&conn);
    let personality_stats = db::get_personality_stats(&conn);
    for stat in personality_stats {
        println!("{} {}", stat.0, stat.1);
    }

    let config = Arc::new(Mutex::new(Config {
        db_conn: conn,
        seen_faces: HashSet::new(),
    }));

    websocket(Arc::clone(&config));


    rocket::ignite()
        .mount("/", routes![new, get])
        .mount("/static", routes![index, files])
        .catch(catchers![not_found])
        .manage(Arc::clone(&config))
        .launch();
}
