use std::io::Read;
use std::collections::HashSet;

use std::path::Path;
use std::sync::{Mutex, Arc};
use std::thread;
use ws::listen;
use rocket::response::NamedFile;

use rocket::State;
use rocket_contrib::{Json, Value};
use rocket::http::RawStr;

use inflector::cases::titlecase::to_title_case;

use super::face;
use super::db;
use super::email;
use super::google;

use rusqlite::Connection;
use ws::Message;

pub struct BackendState {
    pub db_conn: Connection,
    pub seen_faces: HashSet<face::FaceId>,
}

type ServerState = Arc<Mutex<BackendState>>;


#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/map")]
fn map() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/map.html")).ok()
}

#[get("/stats")]
fn stats() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/stats.html")).ok()
}


#[get("/essay/<personality>")]
fn essay(personality: &RawStr, conf: State<ServerState>) -> Json<Value> {
    let conf = &mut conf.lock().expect("get conf");
    let personality: String = to_title_case(personality);
    db::increment_personality(&conf.db_conn, personality.to_string());

    let personality_stats = db::get_personality_stats(&conf.db_conn);
    for stat in personality_stats {
        println!("{} {}", stat.0, stat.1);
    }
    println!();

    let mut essay = String::new();
    match NamedFile::open(Path::new(&format!("static/essays/{}.html", personality))) {
        Ok(mut file) => {
            file.read_to_string(&mut essay);
        }
        Err(e) => {
            error!("{:?}", e);
            NamedFile::open(Path::new("static/essays/Open.html")).unwrap().read_to_string(&mut essay);
        }
    }
    Json(json!([essay]))
}

#[post("/face", format = "application/json", data = "<face_json>")]
fn new_face(face_json: Json<face::Face>, conf: State<ServerState>) -> Json<Value> {
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

//#[get("/face/<id>", format = "application/json")]
//fn get(id: face::FaceId, conf: State<ServerConfig>) -> Option<Json<db::FaceEvent>> {
//    let db_conn = &conf.lock().expect("get conf").db_conn;
//    Some(Json(db::get_face(db_conn, id)))
//}

#[derive(FromForm, Debug)]
pub struct Consent {
    pub from_name: String,
    pub access_token: String,
    pub personality: String,
}

#[get("/email?<consent>", format = "application/json")]
fn emails(consent: Consent, conf: State<ServerState>) -> Json<Value> {
    println!("{:?}", consent);
    email::send_emails(&consent);
    Json(json!({ "status": "ok" }))
}


#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error-404",
        "reason": "Resource was not found."
    }))
}


fn websocket(conf: ServerState) {
    let conf_clone = Arc::clone(&conf);
    thread::spawn(|| {
        listen("127.0.0.1:3012", move |out| {
            let conf_clone = Arc::clone(&conf_clone);
            move |msg: Message| {
                match msg.into_text() {
                    Ok(s) =>
                        match s.as_ref() {
                            "map" => {
                                out.send(google::get_analytics())
                            }
                            "personality-stats" => {
                                let db_conn = &conf_clone.lock().expect("get conf").db_conn;
                                let personalities = db::get_personality_stats(db_conn);
                                let serialized_personalities = serde_json::to_string(&personalities).unwrap();
                                out.send(serialized_personalities)
                            }
                            _ => {
                                out.send("")
                            }
                        }
                    Err(_) => {
                        out.send("")
                    }
                }
            }
        })
    });
}

pub fn start_api(conf: ServerState) {
    websocket(Arc::clone(&conf));

    rocket::ignite()
        .mount("/", routes![index, stats, map])
        .mount("/api", routes![new_face, essay, emails])
        .catch(catchers![not_found])
        .manage(Arc::clone(&conf))
        .launch();
}