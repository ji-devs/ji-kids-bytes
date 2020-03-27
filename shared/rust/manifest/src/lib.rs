use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppManifest {
    pub topics: Vec<Meta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicManifest {
    pub meta: Meta,
    pub videos: Vec<Media>,
    pub games: Vec<Media>,
    pub discovers: Vec<Link>,
    pub create: Create,
    pub crafts: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub id: String,
    pub title: String,
    pub locked: bool,
}

//used for both watch and games
#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub id: String,
    pub player: MediaPlayer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MediaPlayer {
    Youtube,
    JiTap,
}

impl <T: AsRef<str>> From<T> for MediaPlayer {
    fn from(text: T) -> Self {
        match text.as_ref() {
            "youtube" => Self::Youtube,
            "jitap" => Self::JiTap,
            _ => unimplemented!("unknown media player: {}", text.as_ref())
        }
    }
}

//Used for both Discover and Craft
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub link: String,

    pub image_filename: String,

    pub link_label: Option<String>,

    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    pub tool: CreationTool,

    pub image_filename: String,

    pub title: String,

    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CreationTool {
    JiTap,
    JiStudio,
    Spark,
}
impl <T: AsRef<str>> From<T> for CreationTool {
    fn from(text: T) -> Self {
        match text.as_ref() {
            "jitap" => Self::JiTap,
            "jistudio" => Self::JiStudio,
            "spark" => Self::Spark,
            _ => unimplemented!("unknown creation tool: {}", text.as_ref())
        }
    }
}