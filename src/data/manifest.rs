use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppManifest {
    pub series: Vec<Series>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Series {
    pub id: String,
    pub title: String,
    pub topics: Vec<TopicMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicManifest {
    pub meta: TopicMeta,
    pub videos: Vec<Media>,
    pub games: Vec<Media>,
    pub discovers: Vec<Link>,
    pub create: Create,
    pub crafts: Vec<Link>,
}

impl From<TopicManifest> for TopicManifestWithAlbum {
    fn from(manifest:TopicManifest) -> Self {
        let TopicManifest {meta, videos, games, discovers, create, crafts} = manifest;

        Self {
            meta,
            videos: videos.into_iter().map(|x| x.into()).collect(),
            games: games.into_iter().map(|x| x.into()).collect(),
            discovers,
            create,
            crafts
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TopicManifestWithAlbum {
    pub meta: TopicMeta,
    pub videos: Vec<MediaWithAlbum>,
    pub games: Vec<MediaWithAlbum>,
    pub discovers: Vec<Link>,
    pub create: Create,
    pub crafts: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopicMeta {
    pub id: String,
    pub title: String,
    pub series_id: String,
    pub series_title: String,
    pub locked: bool,
}

//used for both watch and games
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub id: String,
    pub player: MediaPlayer,
}

impl From<Media> for MediaWithAlbum {
    fn from(media:Media) -> Self {
        let Media {id, player} = media;

        if player == MediaPlayer::JiTap {
            //tiny tap's algorithm...
            let album_id:&str = &id[1..];
            let mut album_id:u64 = u64::from_str_radix(album_id, 36).expect("parse ji tap album id");
            album_id -= 1234;

            Self {
                id,
                album_id: album_id.to_string(), 
                player
            }
        } else {
            Self {
                id,
                album_id: "".to_string(),
                player
            }
        }
    }
}

//used for both watch and games
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaWithAlbum {
    pub id: String,
    pub album_id: String,
    pub player: MediaPlayer,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
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
    Sketchpad,
    Autodraw
}
impl <T: AsRef<str>> From<T> for CreationTool {
    fn from(text: T) -> Self {
        match text.as_ref() {
            "jitap" => Self::JiTap,
            "jistudio" => Self::JiStudio,
            "spark" => Self::Spark,
            "sketchpad" => Self::Sketchpad,
            "autodraw" => Self::Autodraw,
            _ => unimplemented!("unknown creation tool: {}", text.as_ref())
        }
    }
}