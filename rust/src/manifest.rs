use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub meta: Meta,
    pub videos: Vec<Video>,
    pub games: Vec<Game>,
    pub discovers: Vec<Discover>,
    pub creates: Vec<Create>,
    pub crafts: Vec<Craft>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Discover {
    pub link: String,

    pub image_filename: String,

    pub title: String,

    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Create {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Craft {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}