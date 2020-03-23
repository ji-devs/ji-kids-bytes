use wasm_bindgen::prelude::*;
use awsm_web::loaders::fetch;
use crate::manifest::*;
use crate::path::*;

pub async fn load_topic_manifest(id:&str) -> Result<TopicManifest, JsValue> {

    let url = media_url(&format!("manifests/topics/{}.json?cb={}", id, js_sys::Date::now()));

    let manifest:TopicManifest = fetch::json(&url).await?;
    Ok(manifest)
}


pub async fn load_app_manifest() -> Result<AppManifest, JsValue> {

    let url = media_url(&format!("manifests/app.json?cb={}", js_sys::Date::now()));

    let manifest:AppManifest = fetch::json(&url).await?;
    Ok(manifest)
}