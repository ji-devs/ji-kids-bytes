use std::rc::{Rc};
use std::cell::{RefCell};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use awsm_web::loaders::fetch;
use gloo_events::EventListener;
use std::future::Future;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use crate::manifest::*;
use crate::path::*;

pub async fn load_manifest(id:&str) -> Result<Manifest, JsValue> {
    let url = media_url(&format!("manifests/{}.json", id));

    let manifest:Manifest = fetch::json(&url).await?;
    Ok(manifest)
}