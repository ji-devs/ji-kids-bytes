use std::sync::Arc;
use std::str::FromStr;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use crate::settings::{SETTINGS, META_BASE_URL, META_BASE_DESCRIPTION, META_BASE_TITLE};
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::data::manifest::{TopicManifest, TopicManifestWithAlbum, Series, TopicMeta};
use crate::loader::{load_string, load_json};

#[derive(Eq, PartialEq, Copy, Clone, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TopicSection {
    Watch,
    Games,
    Discover,
    Create,
    Craft
}

pub async fn player_page(hb:Arc<Handlebars<'_>>, section:Option<TopicSection>, topic_name:String) -> Result<impl warp::Reply, warp::Rejection> {

    let manifest:TopicManifest = load_json(&SETTINGS.path_topic_manifest(&topic_name)).await?;

    let manifest:TopicManifestWithAlbum = manifest.into();

    //social tags are based on real url - i.e. "/" isn't turned to "/watch"
    let social = Social::new(&manifest.meta, section);

    //playing needs a real section, so default to "watch"
    let section = section.unwrap_or(TopicSection::Watch);

    let render = hb
        .render("player", &PlayerData {
            path_ui: SETTINGS.path_ui(),
            social, 
            manifest, 
            section: section.to_string(),
            local_dev: SETTINGS.local_dev
        })
        .unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}

#[derive(Serialize)]
pub struct PlayerData {
    #[serde(rename="PathUi")]
    path_ui: String,
    social: Social,
    manifest: TopicManifestWithAlbum,
    section: String,
    local_dev: bool
}

#[derive(Serialize)]
struct Social {
    title: String,
    url: String,
    image: String,
    description: String
}

impl Social {
    pub fn new(topic: &TopicMeta, section:Option<TopicSection>) -> Self {

        let url = match section {
            Some(section) => format!("{}/topic/{}/{}", META_BASE_URL, topic.id, section.to_string()),
            None => format!("{}/topic/{}", META_BASE_URL, topic.id),
        };

        Self{
            title: format!("{} - {}", topic.title, META_BASE_TITLE),
            url,
            image: SETTINGS.path_topic(&format!("{}_small.png", topic.id), &topic.id),
            description: format!("{} - {}", topic.title, META_BASE_DESCRIPTION)
        }
    }
}

pub async fn player_page_section_str(hb:Arc<Handlebars<'_>>, section_name:String, topic_name:String) -> Result<impl warp::Reply, warp::Rejection> {
    match TopicSection::from_str(&section_name) {
        Ok(section) => player_page(hb, Some(section), topic_name).await,
        Err(_) => Err(warp::reject::not_found())
    }
}

