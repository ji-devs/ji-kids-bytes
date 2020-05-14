use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use warp::{
    reply::Response,
    hyper::Body,
    Reply,
    http::header::{HeaderName, HeaderValue, CONTENT_TYPE},
};
use super::player::TopicSection;
use crate::settings::{SETTINGS, META_BASE_URL};
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::data::manifest::{AppManifest, Series, TopicMeta};
use crate::loader::load_json;

#[derive(Serialize)]
struct SiteMap {
    #[serde(rename="SiteBase")]
    site_base: String,
    series: Vec<Series>,
}
impl SiteMap {
    pub fn new(manifest:AppManifest) -> Self {
        Self{
            site_base: META_BASE_URL.to_string(),
            series: manifest.series,
        }
    }
}

pub async fn sitemap_page(hb:Arc<Handlebars<'_>>) -> Result<impl Reply, warp::Rejection> {

    let manifest:AppManifest = load_json(&SETTINGS.path_app_manifest()).await?;

    let render = hb
        .render("sitemap", &SiteMap::new(manifest)) 
        .unwrap_or_else(|err| err.to_string());

    let mut res = Response::new(Body::from(render));
    res.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/xml; charset=utf-8"),
    );

    Ok(res)
}
