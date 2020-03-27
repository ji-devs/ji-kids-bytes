use serde::{Deserialize};

#[derive(Deserialize)]
pub struct GoogleSheet {
    pub values: Vec<Vec<String>>
}

pub struct DriveAppManifestRow {
    pub doc_id: String,
    pub locked: bool
}