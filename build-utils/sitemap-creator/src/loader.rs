use manifest::*;

use std::fs::File;
use std::path::Path;
use serde_json;
use crate::config::Config;

pub fn load_main_mainfest(config:&Config) -> AppManifest {
    let json_file_path = Path::new(&config.manifest_dir).join("app.json");
    let json_file = File::open(json_file_path).expect("main manifest not found");
    serde_json::from_reader(json_file).expect("error while reading main manifest json")
}