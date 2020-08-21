// see https://github.com/rust-lang/rust/issues/70070
#![cfg_attr(feature = "quiet", allow(warnings))]

mod config;
mod schema;
mod loader;
mod writer;
mod manifest;

use structopt::StructOpt;
use loader::*;
use config::*;
use schema::*;
use writer::*;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use manifest::*;

#[tokio::main]
async fn main() {
    let config = Config::from_args();
    dotenv().ok();

    let api_key = env::var("GOOGLE_API_KEY").expect("requires GOOGLE_API_KEY in env");

    //1. Load Sheets 
    eprintln!("----main app manifest----");
    let manifest_list:Vec<DriveAppManifestRow> = load_manifest_list(&config.manifest_list_id, &api_key, &config).await;

    let mut series_lookup = HashMap::<String, Vec<TopicMeta>>::new();
    let mut series_order = Vec::<String>::new();

    for drive_topic_meta in manifest_list.iter() {
        let DriveAppManifestRow { doc_id, series_id, locked, ..} = drive_topic_meta;
        if !series_lookup.contains_key(series_id) {
            series_order.push(series_id.to_string());
        }

        eprintln!("----topic manifest {}----", doc_id);
        eprintln!("Google doc: https://docs.google.com/spreadsheets/d/{}", doc_id);
        let topics = series_lookup.entry(series_id.to_string()).or_insert(Default::default());
        let meta = load_topic_meta(&drive_topic_meta, &api_key, &config).await;
        topics.push(meta.clone());
      
        if config.only_locked && !locked {
            eprintln!("----SKIPPING {} (only locked)----", meta.id);
        } else {
            eprintln!("----LOADING id {}----", meta.id);
            let videos = load_topic_media(doc_id, "Watch", &api_key, &config).await;
            let games = load_topic_media(doc_id, "Games", &api_key, &config).await;
            let discovers = load_topic_links(doc_id, "Discover", true, &api_key, &config).await;
            let create = load_topic_create(doc_id, &api_key, &config).await;
            let crafts = load_topic_links(doc_id, "Craft", false, &api_key, &config).await;


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

        }
    }

    let mut manifest_path = config.local_output.clone();
    manifest_path.push("app.json");
    
    eprintln!("writing final manifest to {:?}", manifest_path);

    let series:Vec<Series> = 
        series_order
            .into_iter()
            .map(|key| {
                let topics = series_lookup.get(&key).expect("topic must exist in lookup!").clone();
                let TopicMeta { series_id, series_title, ..} = &topics[0];

                if &key != series_id {
                    panic!("uhhhh... something went very wrong!");
                }

                eprintln!("series {} has {} topics", series_id, topics.len());

                Series {
                    id: series_id.clone(),
                    title: series_title.clone(),
                    topics,
                }
            })
            .collect();

    let app_manifest = AppManifest { series };

    write_json(&app_manifest, &manifest_path, config.dry_run);
    eprintln!("manifest written to {:?}", manifest_path);

}
