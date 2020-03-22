use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub meta: Meta,
    pub videos: Vec<Video>,
    pub games: Vec<Game>,
    pub look_closers: Vec<LookCloser>,
    pub creates: Vec<Create>,
    pub hands_ons: Vec<HandsOn>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub id: String,
    pub title: String,
    pub color: String
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
pub struct LookCloser {
    pub link: String,

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
pub struct HandsOn {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}

type DriveSources = (MetaEntry,Vec<WatchEntry>, Vec<GamesEntry>, Vec<LookCloserEntry>, Vec<CreateEntry>, Vec<HandsOnEntry>);
impl From<DriveSources> for Manifest {
    fn from(sources:DriveSources) -> Self {

        let meta = Meta {
            id: sources.0.id.text,
            title: sources.0.title.text,
            color: sources.0.color.text,
        };

        let videos:Vec<Video> = sources.1.into_iter().map(|x| Video {
            id: x.youtube_id.text
        }).collect();

        let games:Vec<Game> = sources.2.into_iter().map(|x| Game {
            id: x.jitap_id.text
        }).collect();

        let look_closers:Vec<LookCloser> = sources.3.into_iter().map(|x| LookCloser {
            link: x.link.text,
            title: x.title.text,
            desc: x.description.text,
        }).collect();

        let creates:Vec<Create> = sources.4.into_iter().map(|x| Create {
            link: x.link.text,
            image_filename: x.image_filename.text,
            header: x.header.text,
            body: x.body.text,
        }).collect();

        let hands_ons:Vec<HandsOn> = sources.5.into_iter().map(|x| HandsOn {
            link: x.link.text,
            image_filename: x.image_filename.text,
            header: x.header.text,
            body: x.body.text,
        }).collect();

        Self {
            meta,
            videos,
            games,
            look_closers,
            creates,
            hands_ons
        }
    }
}