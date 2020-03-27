// see https://github.com/rust-lang/rust/issues/70070
#![cfg_attr(feature = "quiet", allow(warnings))]

mod config;
mod schema;
mod loader;
mod writer;

use structopt::StructOpt;
use loader::*;
use config::*;
use schema::*;
use writer::*;
use dotenv::dotenv;
use std::env;
use manifest::*;

#[tokio::main]
async fn main() {
    let config = Config::from_args();
    dotenv().ok();

    let api_key = env::var("GOOGLE_API_KEY").expect("requires GOOGLE_API_KEY in env");

    //1. Load Sheets 
    eprintln!("----main app manifest----");
    let manifest_list:Vec<DriveAppManifestRow> = load_manifest_list(&config.manifest_list_id, &api_key).await;

    let mut topics:Vec<Meta> = Vec::new();

    for drive_topic_meta in manifest_list.iter() {
        let DriveAppManifestRow { doc_id, ..} = drive_topic_meta;
        eprintln!("----topic manifest {}----", doc_id);
        eprintln!("Google doc: https://docs.google.com/spreadsheets/d/{}", doc_id);
       
        let meta = load_topic_meta(&drive_topic_meta, &api_key).await;
        eprintln!("----topic id {}----", meta.id);
        let videos = load_topic_media(doc_id, "Watch", &api_key).await;
        let games = load_topic_media(doc_id, "Games", &api_key).await;
        let discovers = load_topic_links(doc_id, "Discover", true, &api_key).await;
        let create = load_topic_create(doc_id, &api_key).await;
        let crafts = load_topic_links(doc_id, "Craft", false, &api_key).await;


        let topic_manifest = TopicManifest {
            meta: meta.clone(),
            videos,
            games,
            discovers,
            create,
            crafts
        };

        let mut manifest_path = config.local_output.clone();
        manifest_path.push("topics");
        manifest_path.push(format!("{}.json", topic_manifest.meta.id));

        write_json(&topic_manifest, &manifest_path, config.dry_run);
        eprintln!("manifest for {} topic written to {:?}", topic_manifest.meta.id, manifest_path);

        topics.push(meta);
    }

    let app_manifest = AppManifest { topics };

    let mut manifest_path = config.local_output.clone();
    manifest_path.push("app.json");
    write_json(&app_manifest, &manifest_path, config.dry_run);
    eprintln!("manifest for {} topics written to {:?}", app_manifest.topics.len(), manifest_path);

}