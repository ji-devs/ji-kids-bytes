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

    let mut topics:Vec<TopicMeta> = Vec::new();

    for (index, sheet_id) in manifest_list.iter().enumerate() {
        println!("{}", sheet_id);
        let mut meta:Vec<MetaEntry> = load_manifest_sheet(&sheet_id, 1).await;
        let meta = meta.remove(0);

        topics.push(TopicMeta {
           id: meta.id.text.clone(),
           title: meta.title.text.clone(),
        });

        let watch:Vec<WatchEntry> = load_manifest_sheet(&sheet_id, 2).await;
        let games:Vec<GamesEntry> = load_manifest_sheet(&sheet_id, 3).await;
        let discovers:Vec<DiscoverEntry> = load_manifest_sheet(&sheet_id, 4).await;
        let create:Vec<CreateEntry> = load_manifest_sheet(&sheet_id, 5).await;
        let crafts:Vec<CraftEntry> = load_manifest_sheet(&sheet_id, 6).await;

        let topic_manifest = TopicManifest::from((meta, watch, games, discovers, create, crafts));

        let mut manifest_path = config.local_output.clone();
        manifest_path.push("topics");
        manifest_path.push(format!("{}.json", topic_manifest.meta.id));

        write_json(&topic_manifest, &manifest_path, config.dry_run);
        eprintln!("manifest for {} topic written to {:?}", topic_manifest.meta.id, manifest_path);
    }

    let app_manifest = AppManifest { topics };

    let mut manifest_path = config.local_output.clone();
    manifest_path.push("app.json");
    write_json(&app_manifest, &manifest_path, config.dry_run);
    eprintln!("manifest for {} topics written to {:?}", app_manifest.topics.len(), manifest_path);


}