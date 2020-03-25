use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppManifest {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopicMeta {
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
            _ => unimplemented!()
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
}
impl <T: AsRef<str>> From<T> for CreationTool {
    fn from(text: T) -> Self {
        match text.as_ref() {
            "youtube" => Self::JiTap,
            "jitap" => Self::JiStudio,
            _ => unimplemented!()
        }
    }
}

//combined MetaEntry from the topic's sheet and additional metadata from the top-level list
//right now just lock info

type DriveSources = (TopicMeta,Vec<WatchEntry>, Vec<GamesEntry>, Vec<DiscoverEntry>, Vec<CreateEntry>, Vec<CraftEntry>);
impl From<DriveSources> for TopicManifest {
    fn from(sources:DriveSources) -> Self {

        let meta = sources.0;

        let videos:Vec<Media> = sources.1.into_iter().map(|x| Media {
            id: x.id.text,
            player: x.player.text.into()
        }).collect();

        let games:Vec<Media> = sources.2.into_iter().map(|x| Media {
            id: x.id.text,
            player: x.player.text.into()
        }).collect();

        let discovers:Vec<Link> = sources.3.into_iter().map(|x| Link {
            link: x.link.text,
            image_filename: x.image_filename.text,
            link_label: Some(x.link_label.text),
            title: x.title.text,
        }).collect();

        let x = sources.4.get(0).unwrap();
        let tool = match x.tool.text.as_ref() {
            "jitap" => Some(CreationTool::JiTap),
            "jistudio" =>  Some(CreationTool::JiStudio),
            _ => None
        }.expect(&format!("{} is unknown creation tool!", x.tool.text));


        let create:Create = Create {
            tool,
            image_filename: x.image_filename.text.clone(),
            title: x.title.text.clone(),
            body: x.body.text.clone(),
        };



        let crafts:Vec<Link> = sources.5.into_iter().map(|x| Link {
            link: x.link.text,
            image_filename: x.image_filename.text,
            title: x.title.text,
            link_label: None,
        }).collect();

        Self {
            meta,
            videos,
            games,
            discovers,
            create,
            crafts
        }
    }
}