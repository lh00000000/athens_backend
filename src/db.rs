use rusqlite::{Connection, Error as RusqliteError};
use std::collections::HashMap;

use super::face::Face;

#[derive(Debug)]
pub struct FaceEvent {
    id: i32,
    face_id: i32,
    time_stamp: f32,
}

#[derive(Debug)]
pub struct Personality {
    id: i32,
    personality: String,
    count: i32,
}

pub fn create_db(conn: &Connection) {
    if let Err(e) = conn.execute(
        "
            CREATE TABLE face_event (
            id INTEGER PRIMARY KEY,
            face_id INTEGER KEY,
            time_stamp REAL);
            ", &[]) {
        match e {
            RusqliteError::SqliteFailure(e, o) => match o {
                Some(s) => if s != "table face_event already exists" { panic!(s) },
                None => panic!("no message. error: {}", e)
            }
            _ => panic!("{:?}", e)
        }
    }

    if let Err(e) = conn.execute(
        "
            CREATE TABLE personality (
            id INTEGER PRIMARY KEY,
            personality_type TEXT,
            count INTEGER
            );
            ", &[]) {
        match e {
            RusqliteError::SqliteFailure(e, o) => match o {
                Some(s) => if s != "table personality already exists" { panic!(s) },
                None => panic!("no message. error: {}", e)
            }
            _ => panic!("{:?}", e)
        }
    } else {
        conn.execute_batch(
            "
                INSERT INTO personality (personality_type, count) VALUES ('Open', 0);
                INSERT INTO personality (personality_type, count) VALUES ('Conscientious', 0);
                INSERT INTO personality (personality_type, count) VALUES ('Extroverted', 0);
                INSERT INTO personality (personality_type, count) VALUES ('Agreeable', 0);
                INSERT INTO personality (personality_type, count) VALUES ('Neurotic', 0);
                "
        ).unwrap();
    }

    if let Err(e) = conn.execute(
        "
            CREATE TABLE face_personality (
            face_id INTEGER,
            personality_id INTEGER,
            FOREIGN KEY(face_id) REFERENCES face_event(face_id),
            FOREIGN KEY(personality_id) REFERENCES personality(id)
            );
            ", &[]) {
        match e {
            RusqliteError::SqliteFailure(e, o) => match o {
                Some(s) => if s != "table face_personality already exists" { panic!(s) },
                None => panic!("no message. error: {}", e)
            }
            _ => panic!("{:?}", e)
        }
    }
}

pub fn get_personality_stats(conn: &Connection) -> HashMap<String, i32> {
    let mut stmt = conn.prepare("SELECT id, personality_type, count FROM personality").unwrap();
    let personality_iter = stmt.query_map(&[], |row| {
        Personality {
            id: row.get(0),
            personality: row.get(1),
            count: row.get(2),
        }
    }).unwrap();

    let mut personalities = HashMap::new();


    for personality in personality_iter {
        let personality = personality.unwrap();
        personalities.insert(
            personality.personality,
            personality.count,
        );
    }

    return personalities;
}

pub fn insert_face(conn: &Connection, face: Face) {
    let mut personalities_ids: Vec<i32> = Vec::new();
    for personality in face.personalities() {
        conn.execute("UPDATE personality SET count = count + 1 WHERE personality_type = ?1", &[&personality.to_string()]);
        conn.query_row(
            "SELECT id FROM personality WHERE personality_type = ?1",
            &[&personality.to_string()],
            |row| personalities_ids.push(row.get(0))
        );
    }

    conn.execute("INSERT INTO face_event (face_id, time_stamp) VALUES (?1, ?2)", &[&face.face_id, &face.time_stamp]);
    for personality_id in personalities_ids {
        conn.execute("INSERT INTO face_personality (face_id, personality_id) VALUES (?1, ?2)", &[&face.face_id, &personality_id]);
    }
}