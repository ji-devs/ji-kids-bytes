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
    pub creates: Vec<Create>,
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

    pub title: String,

    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Craft {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}

type DriveSources = (MetaEntry,Vec<WatchEntry>, Vec<GamesEntry>, Vec<DiscoverEntry>, Vec<CreateEntry>, Vec<CraftEntry>);
impl From<DriveSources> for TopicManifest {
    fn from(sources:DriveSources) -> Self {

        let meta = TopicMeta {
            id: sources.0.id.text,
            title: sources.0.title.text,
        };

        let videos:Vec<Video> = sources.1.into_iter().map(|x| Video {
            id: x.youtube_id.text
        }).collect();

        let games:Vec<Game> = sources.2.into_iter().map(|x| Game {
            id: x.jitap_id.text
        }).collect();

        let discovers:Vec<Discover> = sources.3.into_iter().map(|x| Discover {
            link: x.link.text,
            image_filename: x.image_filename.text,
            title: x.title.text,
            desc: x.description.text,
        }).collect();

        let creates:Vec<Create> = sources.4.into_iter().map(|x| Create {
            link: x.link.text,
            image_filename: x.image_filename.text,
            header: x.header.text,
            body: x.body.text,
        }).collect();

        let crafts:Vec<Craft> = sources.5.into_iter().map(|x| Craft {
            link: x.link.text,
            image_filename: x.image_filename.text,
            header: x.header.text,
            body: x.body.text,
        }).collect();

        Self {
            meta,
            videos,
            games,
            discovers,
            creates,
            crafts
        }
    }
}