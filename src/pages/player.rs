use std::sync::Arc;
use std::str::FromStr;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use crate::settings::{SETTINGS, META_BASE_URL, META_BASE_DESCRIPTION, META_BASE_TITLE};
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::data::manifest::{TopicManifest, TopicManifestWithAlbum, Series, TopicMeta, MediaWithAlbum};
use crate::loader::{load_string, load_json};

#[derive(Eq, PartialEq, Copy, Clone, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TopicSection {
    Watch,
    Games,
    Discover,
    Create,
    Craft,
}

pub async fn player_page(hb:Arc<Handlebars<'_>>, section:Option<TopicSection>, topic_name:String, media_index: Option<usize>) -> Result<impl warp::Reply, warp::Rejection> {

    let manifest:TopicManifest = load_json(&SETTINGS.path_topic_manifest(&topic_name)).await?;

    let manifest:TopicManifestWithAlbum = manifest.into();

    //social tags are based on real url - i.e. "/" isn't turned to "/watch"
    let social_section = section.clone();

    //playing needs a real section, so default to "watch"
    let section = section.unwrap_or(TopicSection::Watch);


    let (media, media_index) = {
        if section == TopicSection::Watch || section == TopicSection::Games {
            let media_index = media_index.unwrap_or(0);
            if section == TopicSection::Watch {
                if media_index > manifest.videos.len()-1 {
                    return Err(warp::reject::not_found());
                }
                (Some(manifest.videos[media_index].clone()), Some(media_index))
            } else {
                if media_index > manifest.games.len()-1 {
                    return Err(warp::reject::not_found());
                }
                (Some(manifest.games[media_index].clone()), Some(media_index))
            }
        } else {
            (None, None)
        }
    };

    let social = Social::new(&manifest.meta, social_section, &media, &media_index);

    let render = hb
        .render("player", &PlayerData {
            path_ui: SETTINGS.path_ui(),
            path_topic: SETTINGS.path_topic(&topic_name),
            topic: topic_name,
            social, 
            manifest, 
            media,
            index: media_index,
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
    #[serde(rename="PathTopic")]
    path_topic: String,
    topic: String,
    media: Option<MediaWithAlbum>,
    index: Option<usize>,
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
    pub fn new(topic: &TopicMeta, section:Option<TopicSection>, media:&Option<MediaWithAlbum>, media_index:&Option<usize>) -> Self {

        let url = match section {
            Some(section) => {
                match media_index {
                    Some(index) => format!("{}/topic/{}/{}/{}", META_BASE_URL, topic.id, section.to_string(), index),
                    None => format!("{}/topic/{}/{}", META_BASE_URL, topic.id, section.to_string()),
                }
            }
            None => format!("{}/topic/{}", META_BASE_URL, topic.id),
        };

        let title = match section {
            Some(section) => {
                match media_index {
                    Some(index) => {
                        if section == TopicSection::Watch {
                            format!("{} - video #{} - {}", topic.title, index+1, META_BASE_TITLE)
                        } else if section == TopicSection::Games {
                            format!("{} - game #{} - {}", topic.title, index+1, META_BASE_TITLE)
                        } else {
                            format!("{} - {} - {}", topic.title, section, META_BASE_TITLE)
                        }
                    }
                    None => format!("{} - {} - {}", topic.title, section, META_BASE_TITLE)
                }
            },
            None => format!("{} - {}", topic.title, META_BASE_TITLE),
        };

        Self{
            title: title.clone(),
            url,
            image: format!("{}/{}_large.png", SETTINGS.path_topic(&topic.id), topic.id),
            description: format!("{}. {}", title, META_BASE_DESCRIPTION)
        }
    }
}

pub async fn player_page_section_str(hb:Arc<Handlebars<'_>>, section_name:String, topic_name:String, media_index:Option<usize>) -> Result<impl warp::Reply, warp::Rejection> {
    match TopicSection::from_str(&section_name) {
        Ok(section) => player_page(hb, Some(section), topic_name, media_index).await,
        Err(_) => Err(warp::reject::not_found())
    }
}

