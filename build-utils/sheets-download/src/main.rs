mod config;
mod schema;
mod loader;
mod writer;

use structopt::StructOpt;
use loader::*;
use config::*;
use schema::*;
use writer::*;
use tokio::stream::{iter, Stream, StreamExt};
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let config = Config::from_args();

    //1. Load Sheets 
    let manifest_list = load_manifest_list(&config.manifest_list_id).await;

    for (index, sheet_id) in manifest_list.iter().enumerate() {
        println!("{}", sheet_id);
        let mut meta:Vec<MetaEntry> = load_manifest_sheet(&sheet_id, 1).await;
        let meta = meta.remove(0);
        let watch:Vec<WatchEntry> = load_manifest_sheet(&sheet_id, 2).await;
        let games:Vec<GamesEntry> = load_manifest_sheet(&sheet_id, 3).await;
        let look_closer:Vec<LookCloserEntry> = load_manifest_sheet(&sheet_id, 4).await;
        let create:Vec<CreateEntry> = load_manifest_sheet(&sheet_id, 5).await;
        let hands_on:Vec<HandsOnEntry> = load_manifest_sheet(&sheet_id, 6).await;

        let app_manifest = Manifest::from((meta, watch, games, look_closer, create, hands_on));

        let mut manifest_path = config.local_output.clone();
        manifest_path.push(format!("{}.json", app_manifest.meta.id));

        eprintln!("{:?}", manifest_path);
        write_json(&app_manifest, &manifest_path, config.dry_run);
    }
}