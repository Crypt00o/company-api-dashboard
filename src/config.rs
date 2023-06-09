use dotenv::dotenv;
use std::collections::HashMap;
use std::env::{set_var, vars};

pub fn config_env() {
    dotenv().ok();

    let mut env_map: HashMap<String, String> = HashMap::<String, String>::new();

    for (key, value) in vars() {
        env_map.insert(key, value);
    }

    if !env_map.contains_key(&"SQLITE_MAIN_DB".to_string()) {
        set_var("SQLITE_MAIN_DB", "./main.db");
    }
}
