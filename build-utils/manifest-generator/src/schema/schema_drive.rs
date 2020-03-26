use super::*;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use url::Url;

#[derive(Deserialize)]
pub struct GoogleSheet {
    pub values: Vec<Vec<String>>
}

pub struct DriveAppManifestRow {
    pub doc_id: String,
    pub locked: bool
}