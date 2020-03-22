use super::*;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use url::Url;

// Manifest


#[derive(Serialize, Deserialize, Debug)]
pub struct ListEntry {
    #[serde(rename = "gsx$sheetids")]
    pub sheet_id: TextCell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaEntry {
    #[serde(rename = "gsx$id")]
    pub id: TextCell,
    #[serde(rename = "gsx$title")]
    pub title: TextCell,
    #[serde(rename = "gsx$color")]
    pub color: TextCell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEntry {
    #[serde(rename = "gsx$youtubeid")]
    pub youtube_id: TextCell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesEntry {
    #[serde(rename = "gsx$jitapid")]
    pub jitap_id: TextCell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LookCloserEntry {
    #[serde(rename = "gsx$link")]
    pub link: TextCell,

    #[serde(rename = "gsx$title")]
    pub title: TextCell,

    #[serde(rename = "gsx$description")]
    pub description: TextCell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEntry {
    #[serde(rename = "gsx$link")]
    pub link: TextCell,

    #[serde(rename = "gsx$imagefilename")]
    pub image_filename: TextCell,

    #[serde(rename = "gsx$headertext")]
    pub header: TextCell,

    #[serde(rename = "gsx$bodytext")]
    pub body: TextCell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HandsOnEntry {
    #[serde(rename = "gsx$link")]
    pub link: TextCell,

    #[serde(rename = "gsx$imagefilename")]
    pub image_filename: TextCell,

    #[serde(rename = "gsx$headertext")]
    pub header: TextCell,

    #[serde(rename = "gsx$bodytext")]
    pub body: TextCell,
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

// Sheet
#[derive(Serialize, Deserialize, Debug)]
pub struct DriveSheet {
}