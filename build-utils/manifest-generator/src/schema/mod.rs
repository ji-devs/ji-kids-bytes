use serde::{Deserialize};

#[derive(Deserialize)]
pub struct GoogleSheet {
    pub values: Vec<Vec<String>>
}

#[derive(Debug)]
pub struct DriveAppManifestRow {
    pub doc_id: String,
    pub series_id: String,
    pub series_title: String, //currently created from id
    pub locked: bool,
    pub should_sync: bool
}
