use web_sys::window;
use crate::config;
use wasm_bindgen::prelude::*;
use serde::{Serialize};

#[derive(Serialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Page <'a> {
    Home(HomeSection),
    Topic(&'a str),
    NotFound,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HomeSection {
    Landing,
    Partners,
    Help,
}

#[wasm_bindgen]
pub fn get_page() -> Result<JsValue, JsValue> {
    let uri_parts = get_uri_parts();

    log::info!("getting page...");
    
    let page:Page = {
        if uri_parts.len() == 0 {
            Page::Home(HomeSection::Landing)
        } else if uri_parts[0] == "help" {
            Page::Home(HomeSection::Help)
        } else if uri_parts[0] == "partners" {
            Page::Home(HomeSection::Partners)
        } else if uri_parts[0] == "topic" {
            if uri_parts.len() > 1 {
                Page::Topic(&uri_parts[1])
            } else {
                Page::NotFound
            }
        } else {
            Page::NotFound
        }
    };

    serde_wasm_bindgen::to_value(&page).map_err(|err| err.into())
}

fn get_uri_parts() -> Vec<String> {
    match window().unwrap().location().pathname() {
        Ok(pathname) => {
            let uri = get_root(pathname.as_str());
            if uri == "" {
                vec![]
            } else {
                uri.split("/").map(|s| s.to_string()).collect()
            }
        },
        Err(_) => vec![]
    }
}



/*
pub async fn get_page() -> Option<RootPage> {
    let uri_parts = get_uri_parts();
    if uri_parts.len() == 0 {
        let app_manifest = load_app_manifest().await.expect("unable to load app manifest!!");
        Some(RootPage::Home(Rc::new(app_manifest)))

    } else if uri_parts[0] == "topic" {
        get_topic(&uri_parts)
            .await
            .map(|manifest| RootPage::Topic(Rc::new(manifest)))
    } else {
        None
    }
}
*/

//simple stripping of host dir like if deploying to example.com/foo
fn get_root(input: &str) -> &str {
    let stripped = match config::HOST_DIRECTORY {
        None => input,
        Some(host_dir) => {
            input
                .find(host_dir)
                .map(|len| input.split_at(len + host_dir.len() - 1).1)
                .or(Some(input))
                .unwrap()
        }
    };

    stripped.trim_matches('/')
}