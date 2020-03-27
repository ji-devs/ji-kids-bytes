use std::fs::{self, File};
use std::path::{Path};
use std::io::Write;

pub fn write_json<J: serde::Serialize>(json:&J, path:&Path, dry_run: bool) {
    if !dry_run {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let file = File::create(path).unwrap();

        serde_json::to_writer_pretty(file, json).unwrap();
    }
}

pub fn write_html(html:&str, path:&Path, dry_run:bool) {
    if !dry_run {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(html.as_bytes()).unwrap();

    }
}