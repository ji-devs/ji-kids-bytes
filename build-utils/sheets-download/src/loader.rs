use url::Url;
use crate::schema::*;
use std::fs::File;
use std::path::Path;
use url::ParseError;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum UrlOrPath<'a> {
    Url(Url),
    Path(&'a Path)
}

pub fn sheet_url(id:&str, sheet_num:u32) -> Result<Url, ParseError> {
    let url = format!("https://spreadsheets.google.com/feeds/list/{}/{}/public/values?alt=json", id, sheet_num);
    eprintln!("loading [sheet {}]: {} (sheet number)", sheet_num, url);

    Url::parse(&url)
}

pub async fn load_manifest_list(sheet_id: &str) -> Vec<String> {
    type ListManifest = DriveManifest<ListEntry>;
    let manifest = reqwest::get(sheet_url(&sheet_id, 1).unwrap())
        .await.unwrap()
        .json::<ListManifest>()
        .await.unwrap();


    manifest.feed.entries
        .iter()
        .map(|entry| entry.sheet_id.text.clone())
        .collect()
}

pub async fn load_manifest_sheet<T: DeserializeOwned>(sheet_id: &str, sheet_num: u32) -> Vec<T> {
    let manifest:DriveManifest<T> = reqwest::get(sheet_url(&sheet_id, sheet_num).unwrap())
        .await.unwrap()
        .json::<DriveManifest<T>>()
        .await.unwrap();

    let entries = manifest.feed.entries;

    entries
}