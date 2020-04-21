use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use awsm_web::loaders::fetch;
use manifest::*;
use js_sys::Promise;
use crate::path::*;

async fn _load_topic_manifest(id:&str) -> Result<JsValue, JsValue> {

    let url = media_url(&format!("manifests/topics/{}.json?cb={}", id, js_sys::Date::now()));

    let manifest:TopicManifest = fetch::json(&url).await?;
    serde_wasm_bindgen::to_value(&manifest).map_err(|err| err.into())

}
async fn _load_app_manifest() -> Result<JsValue, JsValue> {

    let url = media_url(&format!("manifests/app.json?cb={}", js_sys::Date::now()));

    let manifest:AppManifest = fetch::json(&url).await?;
    serde_wasm_bindgen::to_value(&manifest).map_err(|err| err.into())
}

#[wasm_bindgen]
pub fn load_app_manifest() -> Promise {
    future_to_promise(_load_app_manifest())
}

#[wasm_bindgen]
pub fn load_topic_manifest(id:String) -> Promise {
    future_to_promise(async move {
        let res = _load_topic_manifest(&id).await;
        res
    })
}