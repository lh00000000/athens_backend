#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate ws;
extern crate rusqlite;
extern crate bincode;

use rocket_contrib::{Json, Value};
use std::io;
use std::str;
use std::fmt;
use std::time::Duration;
use std::rc::Rc;

use bincode::{serialize, deserialize};
use std::path::{Path, PathBuf};
use rocket::State;
use std::sync::{Mutex, Arc};
use std::thread;
use ws::listen;
use ws::Message;
use rocket::response::NamedFile;

use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use rusqlite::{Connection, Error as RusqliteError};

//mod face;


//struct Config {
//    unqlite_conn: UnQLite,
//}
//
//type ID = String;
//type ServerConfig = Arc<Mutex<Config>>;
//
//#[get("/")]
//fn index() -> io::Result<NamedFile> {
//    NamedFile::open("static/index.html")
//}
//
//#[get("/<file..>")]
//fn files(file: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("static/").join(file)).ok()
//}
//
//#[post("/face/<id>", format = "application/json", data = "<face_json>")]
//fn new(id: ID, face_json: Json<face::Face>, conf: State<ServerConfig>) -> Json<Value> {
//    let conf = conf.lock().expect("map lock.");
//    match conf.unqlite_conn.kv_store(id, &face_json.as_vec_u8()) {
//        Ok(()) => Json(json!({ "status": "ok" })),
//        Err(e) => Json(json!({
//                    "status": "error",
//                    "reason": format!("{}", e)
//                }))
//    }
//}
//
//#[get("/face/<id>", format = "application/json")]
//fn get(id: ID, conf: State<ServerConfig>) -> Option<Json<face::Face>> {
//    let conf = conf.lock().unwrap();
//    match conf.unqlite_conn.kv_fetch(&id) {
//        Ok(contents_as_vec) => {
//            let contents_as_face = face::Face::from_vec_u8(&contents_as_vec);
//            Some(Json(contents_as_face))
//        }
//        Err(e) => None
//    }
//}
//
//
//#[catch(404)]
//fn not_found() -> Json<Value> {
//    Json(json!({
//        "status": "error-404",
//        "reason": "Resource was not found."
//    }))
//}
//
//
//fn websocket(conf: ServerConfig) -> Sender<i32> {
//    thread::spawn(|| {
//        listen("127.0.0.1:3012", move |out| {
//            move |msg| {
//                conf.lock().expect("get conf").unqlite_conn.
//                    out.send(Message::Text(format!("{}", 100)))
//            }
//        })
//    });
//
//    tx
//}

//loop {
//println!("hello");
//match mutex_rx_clone.try_lock() {
//Ok(r) => match r.try_recv() {
//Ok(r) => out.send(Message::Text(format!("{}", r))).unwrap(),
//Err(r) => out.send(Message::Text(format!("{}", r))).unwrap(),
//},
//Err(r) => out.send(Message::Text(format!("{}", r))).unwrap()
//}
//}


//fn rocket() -> rocket::Rocket {
//    let config = Arc::new(Mutex::new(Config {
//        unqlite_conn: UnQLite::create("test.db"),
//    }));
//
//    websocket(Arc::clone(&config));
//
//    rocket::ignite()
//        .mount("/", routes![new, get])
//        .mount("/static", routes![index, files])
//        .catch(catchers![not_found])
//        .manage(Arc::clone(&config))
//}

#[derive(Debug)]
struct FaceEvent {
    id: i32,
    face_id: i32,
    time_stamp: f32,
}

#[derive(Debug)]
struct Personality {
    id: i32,
    personality: String,
}

fn create_db(conn: &Connection) {
    if let Err(e) = conn.execute(
        "CREATE TABLE face_event (
             id INTEGER PRIMARY KEY,
             face_id INTEGER KEY,
             time_stamp REAL);", &[]) {
        match e {
            RusqliteError::SqliteFailure(e, o) => match o {
                Some(s) => if s != "table face_event already exists" { panic!(s) },
                None => panic!("no message. error: {}", e)
            }
            _ => panic!("{:?}", e)
        }
    }

    if let Err(e) = conn.execute(
        "CREATE TABLE personality (
             id INTEGER PRIMARY KEY,
             personality_type TEXT
             );", &[]) {
        match e {
            RusqliteError::SqliteFailure(e, o) => match o {
                Some(s) => if s != "table personality already exists" { panic!(s) },
                None => panic!("no message. error: {}", e)
            }
            _ => panic!("{:?}", e)
        }
    }

    if let Err(e) = conn.execute(
        "CREATE TABLE face_personality (
             face_id INTEGER,
             personality_id INTEGER,
             FOREIGN KEY(face_id) REFERENCES face_event(face_id),
             FOREIGN KEY(personality_id) REFERENCES personality(id)
             );", &[]) {
        match e {
            RusqliteError::SqliteFailure(e, o) => match o {
                Some(s) => if s != "table face_personality already exists" { panic!(s) },
                None => panic!("no message. error: {}", e)
            }
            _ => panic!("{:?}", e)
        }
    }
}

fn print_personalities(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, personality_type FROM personality").unwrap();
    let personality_iter = stmt.query_map(&[], |row| {
        Personality {
            id: row.get(0),
            personality: row.get(1),
        }
    }).unwrap();
    for personality in personality_iter {
        println!("Found personality {:?}", personality.unwrap());
    }
}

fn main() {
    let conn = Connection::open("./test.db").unwrap();
//    rocket().launch();
    create_db(&conn);
    print_personalities(&conn);
}
