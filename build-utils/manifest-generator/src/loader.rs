use url::Url;
use crate::schema::*;
use url::ParseError;
use manifest::*;
use std::sync::{Arc, Mutex};
use crate::config::Config;

lazy_static::lazy_static! {
    /// This is an example for using doc comment attributes
    static ref API_CALLS:Arc<Mutex<usize>> = Arc::new(Mutex::new(0)); 
}

pub async fn load_manifest_list(doc_id: &str, api_key:&str, config:&Config) -> Vec<DriveAppManifestRow> {
    load_sheet_rows(doc_id, "Topics", api_key, config)
        .await
        .into_iter()
        .map(|row| {
            let series_id = extract_string(&row, 1).to_lowercase();
            let series_title = some_kind_of_uppercase_first_letter(&series_id);
            DriveAppManifestRow {
                doc_id: extract_string(&row, 0), 
                series_id,
                series_title,
                locked: extract_bool(&row, 2), 
            }
        })
        .collect()
}

pub async fn load_topic_meta(app_manifest_row:&DriveAppManifestRow, api_key:&str, config:&Config) -> TopicMeta {
    let DriveAppManifestRow { doc_id, series_id, series_title, locked} = app_manifest_row;

    load_sheet_rows(&doc_id, "Meta", api_key, config)
        .await
        .into_iter()
        .map(|row| {

            TopicMeta {
                id: extract_string(&row, 0), 
                title: extract_string(&row, 1), 
                series_id: series_id.clone(),
                series_title: series_title.clone(),
                locked: *locked
            }
        })
        .nth(0)
        .unwrap()
}

pub async fn load_topic_media(doc_id:&str, sheet_id:&str, api_key:&str, config:&Config) -> Vec<Media> {
    load_sheet_rows(doc_id, sheet_id, api_key, config)
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

pub async fn load_topic_links(doc_id:&str, sheet_id:&str, has_link_label: bool, api_key:&str, config:&Config) -> Vec<Link> {
    load_sheet_rows(doc_id, sheet_id, api_key, config)
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

pub async fn load_topic_create(doc_id:&str, api_key:&str, config:&Config) -> Create {
    load_sheet_rows(&doc_id, "Create", api_key, config)
        .await
        .into_iter()
        .map(|row| {
            Create {
                tool: extract_string(&row, 0).into(), 
                image_filename: extract_string(&row, 1),
                title: extract_string(&row, 2), 
                body: extract_string(&row, 3), 
            }
        })
        .nth(0)
        .unwrap()
}

async fn load_sheet_rows(doc_id:&str, sheet_id: &str, api_key:&str, config:&Config) -> Vec<Vec<String>> {
    {
        let mut n_calls = API_CALLS.lock().unwrap();
        //could maybe be up to 99 but... play it safe for now :\
        if *n_calls >= config.per_batch {
            println!("WAIT!!! need to sleep in order to not exhaust google api!");
            tokio::time::delay_for(tokio::time::Duration::new(config.batch_sleep_time, 0)).await;
            *n_calls = 0;
        } else {
            *n_calls += 1;
        }

    }

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


//https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}