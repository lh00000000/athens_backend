#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate ws;

use rocket_contrib::{Json, Value};
use std::io;
use std::path::{Path, PathBuf};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use ws::listen;
use rocket::response::NamedFile;

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "application/json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<MessageMap>) -> Json<Value> {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        Json(json!({
            "status": "error",
            "reason": "ID exists. Try put."
        }))
    } else {
        hashmap.insert(id, message.0.contents);
        Json(json!({ "status": "ok" }))
    }
}

#[put("/<id>", format = "application/json", data = "<message>")]
fn update(id: ID, message: Json<Message>, map: State<MessageMap>) -> Option<Json<Value>> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(Json(json!({ "status": "ok" })))
    } else {
        None
    }
}

#[get("/<id>", format = "application/json")]
fn get(id: ID, map: State<MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
            id: Some(id),
            contents: contents.clone(),
        })
    })
}

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![new, update, get])
        .mount("/static", routes![index, files])
        .catch(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    thread::spawn(|| {
        listen("127.0.0.1:3012", |out| {
            move |msg| {
                println!("{}", msg);
                out.send(msg)
            }
        })
    });
    rocket().launch();
}
