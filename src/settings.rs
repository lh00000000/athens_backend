use config::*;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::RwLock;
use super::face;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("settings.toml")).unwrap();

        settings
    });
}

pub fn get_config(key: &str) -> Option<String> {
    let settings = SETTINGS.read()
        .unwrap();

    match settings.clone().try_into::<HashMap<String, String>>().unwrap().get(key) {
        Some(s) => Some(s.clone()),
        None => None
    }
}

pub fn set_config(key: &str, val: &str) {
    SETTINGS.write().unwrap().set(key, val);
}
