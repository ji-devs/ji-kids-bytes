use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub meta: Meta,
    pub videos: Vec<Video>,
    pub games: Vec<Game>,
    pub look_closers: Vec<LookCloser>,
    pub creates: Vec<Create>,
    pub hands_ons: Vec<HandsOn>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub id: String,
    pub title: String,
    pub color: String
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
pub struct LookCloser {
    pub link: String,

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
pub struct HandsOn {
    pub link: String,

    pub image_filename: String,

    pub header: String,

    pub body: String,
}