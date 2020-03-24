use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppManifest {
    pub topics: Vec<TopicMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicManifest {
    pub meta: TopicMeta,
    pub videos: Vec<Video>,
    pub games: Vec<Game>,
    pub discovers: Vec<Discover>,
    pub create: Create,
    pub crafts: Vec<Craft>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicMeta {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Discover {
    pub link: String,

    pub image_filename: String,

    pub link_label: String,

    pub title: String,

    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    pub tool: CreationTool,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum CreationTool {
    JiTap,
    JiStudio,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Craft {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}