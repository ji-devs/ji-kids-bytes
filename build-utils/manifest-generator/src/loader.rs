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

pub async fn load_manifest_list(doc_id: &str, api_key:&str) -> Vec<DriveAppManifestRow> {
    load_sheet_rows(doc_id, "Topics", api_key)
        .await
        .into_iter()
        .map(|row| {
            DriveAppManifestRow {
                doc_id: extract_string(&row, 0), 
                locked: extract_bool(&row, 1), 
            }
        })
        .collect()
}

pub async fn load_topic_meta(app_manifest_row:&DriveAppManifestRow, api_key:&str) -> Meta {
    let DriveAppManifestRow { doc_id, locked} = app_manifest_row;

    load_sheet_rows(&doc_id, "Meta", api_key)
        .await
        .into_iter()
        .map(|row| {
            Meta {
                id: extract_string(&row, 0), 
                title: extract_string(&row, 1), 
                locked: *locked
            }
        })
        .nth(0)
        .unwrap()
}

pub async fn load_topic_media(doc_id:&str, sheet_id:&str, api_key:&str) -> Vec<Media> {
    load_sheet_rows(doc_id, sheet_id, api_key)
        .await
        .into_iter()
        .map(|row| {
            Media {
                id: extract_string(&row, 0), 
                player: extract_string(&row, 1).into()
            }
        })
        .collect()
}

pub async fn load_topic_links(doc_id:&str, sheet_id:&str, has_link_label: bool, api_key:&str) -> Vec<Link> {
    load_sheet_rows(doc_id, sheet_id, api_key)
        .await
        .into_iter()
        .map(|row| {
            Link {
                link: extract_string(&row, 0),
                image_filename: extract_string(&row, 1),
                title: extract_string(&row, 2),
                link_label: if has_link_label { Some(extract_string(&row, 3) ) } else { None }
            }
        })
        .collect()
}

pub async fn load_topic_create(doc_id:&str, api_key:&str) -> Create {
    load_sheet_rows(&doc_id, "Create", api_key)
        .await
        .into_iter()
        .map(|row| {
            Create {
                tool: extract_string(&row, 0).into(), 
                image_filename: extract_string(&row, 1),
                title: extract_string(&row, 2), 
                body: extract_string(&row, 2), 
            }
        })
        .nth(0)
        .unwrap()
}

async fn load_sheet_rows(doc_id:&str, sheet_id: &str, api_key:&str) -> Vec<Vec<String>> {
    reqwest::get(sheet_url(doc_id, sheet_id, api_key).unwrap())
        .await.unwrap()
        .json::<GoogleSheet>()
        .await.unwrap()
        .values
        .into_iter()
        .skip(1)
        .filter(|row| {
            let valid = 
                row.len() > 0 
                && match row.get(0) {
                    None => false,
                    Some(first_cell) => !first_cell.is_empty()
                };
            valid
        })
        .collect()
}

fn extract_string(row: &[String], n: usize) -> String {
    extract_str(row, n).to_string()
}
fn extract_bool(row: &[String], n: usize) -> bool {
    match extract_str(row, n).to_uppercase().as_ref() {
        "TRUE" => true,
        "FALSE" => false,
        _ => unimplemented!()
    }
}

fn extract_str(row: &[String], n: usize) -> &str {
    let value:&str = row.get(n).expect(&format!("column {} is invalid", n));
    if value.is_empty() {
        panic!("column {} is empty", n);
    }

    value
}
fn sheet_url(doc_id:&str, sheet_name:&str, api_key:&str) -> Result<Url, ParseError> {

    let url = format!("https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}", doc_id, sheet_name, api_key);

    eprintln!("loading [{}]: {}", sheet_name, url);

    Url::parse(&url)
}


/*
pub async fn load_manifest_sheet<T: DeserializeOwned + IsEmpty>(sheet_id: &str, sheet_num: u32) -> Vec<T> {
    let manifest:DriveManifest<T> = reqwest::get(sheet_url(&sheet_id, sheet_num).unwrap())
        .await.unwrap()
        .json::<DriveManifest<T>>()
        .await.unwrap();

    manifest.feed.entries
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect()
}
*/