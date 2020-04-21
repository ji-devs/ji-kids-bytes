// see https://github.com/rust-lang/rust/issues/70070
#![cfg_attr(feature = "quiet", allow(warnings))]

mod config;
mod loader;

use structopt::StructOpt;
use loader::*;
use config::*;
use std::env;
use manifest::*;
use std::path::Path;
use std::fs::{File, read_to_string};
use sitemap::writer::SiteMapWriter;
use sitemap::structs::UrlEntry;
use std::io::stdout;
#[tokio::main]
async fn main() {
    let config = Config::from_args();

    let app_manifest = load_main_mainfest(&config);

    let mut urls = vec![config.base_url.to_string()];

    let topics = app_manifest.series.into_iter().flat_map(|series| series.topics);
    for topic in topics.into_iter().filter(|topic| !topic.locked) {
        let TopicMeta { id, title, locked, ..} = topic;

        urls.push(format!("{}/topic/{}", config.base_url, id));
    }

    if !config.dry_run {
        let mut file = File::create(config.sitemap_file).unwrap();
        let sitemap_writer = SiteMapWriter::new(&mut file);
        let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
        
        for url in urls.into_iter() {
            urlwriter.url(url).expect("Unable to write url");
        }
        urlwriter.end().expect("Unable to write close tags");

    } else {
        println!("dry run!");
        for url in urls.into_iter() {
            println!("{}", url);
        }
    }
}