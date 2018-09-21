use config::*;
use rusqlite::Connection;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;
use super::face;


pub struct BackendState {
    pub db_conn: Connection,
    pub seen_faces: HashSet<face::FaceId>,
}

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("settings.toml")).unwrap();

        settings
    });
}

pub fn get_config(key: &str) -> String {
    let settings = SETTINGS.read()
        .unwrap();

    settings.clone().try_into::<HashMap<String, String>>()
        .unwrap()
        .get(key)
        .unwrap()
        .to_string()
}