use super::*;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use url::Url;

// Manifest

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevelMetaEntry {
    #[serde(rename = "gsx$sheetids")]
    pub sheet_id: TextCell,
    #[serde(rename = "gsx$locked")]
    pub locked: TextCell,
}

impl IsEmpty for TopLevelMetaEntry {
    fn is_empty(&self) -> bool {
        self.sheet_id.is_empty()
        || self.locked.is_empty()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TopicMetaEntry {
    #[serde(rename = "gsx$id")]
    pub id: TextCell,
    #[serde(rename = "gsx$title")]
    pub title: TextCell,
}

impl IsEmpty for TopicMetaEntry {
    fn is_empty(&self) -> bool {
        self.id.is_empty()
        || self.title.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEntry {
    #[serde(rename = "gsx$id")]
    pub id: TextCell,
    #[serde(rename = "gsx$player")]
    pub player: TextCell,
}

impl IsEmpty for WatchEntry {
    fn is_empty(&self) -> bool {
        self.id.is_empty()
        || self.player.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesEntry {
    #[serde(rename = "gsx$id")]
    pub id: TextCell,
    #[serde(rename = "gsx$player")]
    pub player: TextCell,
}

impl IsEmpty for GamesEntry {
    fn is_empty(&self) -> bool {
        self.id.is_empty()
        || self.player.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoverEntry {
    #[serde(rename = "gsx$link")]
    pub link: TextCell,

    #[serde(rename = "gsx$imagefilename")]
    pub image_filename: TextCell,

    #[serde(rename = "gsx$linklabel")]
    pub link_label: TextCell,

    #[serde(rename = "gsx$title")]
    pub title: TextCell,

}

impl IsEmpty for DiscoverEntry {
    fn is_empty(&self) -> bool {
        self.link.is_empty()
        || self.image_filename.is_empty()
        || self.link_label.is_empty()
        || self.title.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEntry {
    #[serde(rename = "gsx$tool")]
    pub tool: TextCell,

    #[serde(rename = "gsx$imagefilename")]
    pub image_filename: TextCell,

    #[serde(rename = "gsx$title")]
    pub title: TextCell,

    #[serde(rename = "gsx$bodytext")]
    pub body: TextCell,
}

impl IsEmpty for CreateEntry {
    fn is_empty(&self) -> bool {
        self.tool.is_empty()
        || self.image_filename.is_empty()
        || self.title.is_empty()
        || self.body.is_empty()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CraftEntry {
    #[serde(rename = "gsx$link")]
    pub link: TextCell,

    #[serde(rename = "gsx$imagefilename")]
    pub image_filename: TextCell,

    #[serde(rename = "gsx$title")]
    pub title: TextCell,
}

impl IsEmpty for CraftEntry {
    fn is_empty(&self) -> bool {
        self.link.is_empty()
        || self.image_filename.is_empty()
        || self.title.is_empty()
    }
}

//////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct DriveManifest<E> {
    pub feed: Feed<E>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed<E> {
    #[serde(rename = "entry")]
    pub entries: Vec<E> 
}




#[derive(Serialize, Deserialize, Debug)]
pub struct TextCell {
    #[serde(rename = "$t")]
    pub text: String,
}

impl IsEmpty for TextCell {
    fn is_empty(&self) -> bool {
        self.text == ""
    }
}

// Sheet
#[derive(Serialize, Deserialize, Debug)]
pub struct DriveSheet {
}