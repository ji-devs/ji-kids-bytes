use crate::schema::*;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use url::Url;
use tokio::stream::{iter};
use futures::stream::{StreamExt};
use tokio::io::{AsyncWriteExt};

pub fn write_json<J: serde::Serialize>(json:&J, path:&Path, dry_run: bool) {
    if !dry_run {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let file = File::create(path).unwrap();

        serde_json::to_writer_pretty(file, json).unwrap();
    }
}