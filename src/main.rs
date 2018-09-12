#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate ws;
extern crate unqlite;
extern crate bincode;

use unqlite::{UnQLite, Config, KV, Cursor};
use rocket_contrib::{Json, Value};
use std::io;
use std::str;
use std::fmt;

use bincode::{serialize, deserialize};
use std::path::{Path, PathBuf};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use ws::listen;
use rocket::response::NamedFile;

mod face;

type ID = String;
type UnQLiteConn = Mutex<UnQLite>;


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/face/<id>", format = "application/json", data = "<face_json>")]
fn new(id: ID, face_json: Json<face::Face>, map: State<UnQLiteConn>) -> Json<Value> {
    let mut hashmap = map.lock().expect("map lock.");
    match hashmap.kv_store(id, &face_json.as_vec_u8()) {
        Ok(()) => Json(json!({ "status": "ok" })),
        Err(e) => Json(json!({
            "status": "error",
            "reason": format!("{}", e)
        }))
    }
}

#[get("/face/<id>", format = "application/json")]
fn get(id: ID, map: State<UnQLiteConn>) -> Option<Json<face::Face>> {
    let hashmap = map.lock().unwrap();
    match hashmap.kv_fetch(&id) {
        Ok(contents_as_vec) => {
            let contents_as_face = face::Face::from_vec_u8(&contents_as_vec);
            Some(Json(contents_as_face))
        }
        Err(e) => None
    }
}


#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error-404",
        "reason": "Resource was not found."
    }))
}


fn websocket() {
    thread::spawn(|| {
        listen("127.0.0.1:3012", |out| {
            move |msg| {
                println!("{}", msg);
                out.send(msg)
            }
        })
    });
}

fn rocket() -> rocket::Rocket {
    websocket();
    rocket::ignite()
        .mount("/", routes![new, get])
        .mount("/static", routes![index, files])
        .catch(catchers![not_found])
        .manage(Mutex::new(UnQLite::create("test.db")))
}

fn main() {
    rocket().launch();
}
