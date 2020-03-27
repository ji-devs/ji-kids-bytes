// see https://github.com/rust-lang/rust/issues/70070
#![cfg_attr(feature = "quiet", allow(warnings))]

mod config;
mod loader;
mod writer;
mod replacer;
mod social;

use structopt::StructOpt;
use loader::*;
use config::*;
use writer::*;
use std::env;
use manifest::*;
use std::path::Path;
use std::fs::{File, read_to_string};
use replacer::*;
use social::*;
use writer::*;

#[tokio::main]
async fn main() {
    let config = Config::from_args();

    //Load template html as string
    let file_path = Path::new(&config.html_dir).join("index.html");
    let template= read_to_string(file_path).unwrap();

    //Handle the root index.html
    write_html(
        &replace_social_tags(&template, &SocialTags::new_home(&config)),
        &Path::new(&config.html_dir).join("index.html"),
        config.dry_run
    );

    //Load main manifest
    let app_manifest = load_main_mainfest(&config);
    for topic in app_manifest.topics {
        let Meta { id, title, locked} = topic;

        let social_tags = SocialTags::new(
            &config,
            Some(&title),
            None, //Todo - get description from manifest?
            Some(&format!("social/topics/{}.png", id)),
            Some(&format!("topic/{}", id))
        );
        
        let html_path = &Path::new(&config.html_dir)
            .join("topic")
            .join(id)
            .join("index.html");

        let final_html = replace_social_tags(&template, &social_tags);

        write_html(&final_html, &html_path, config.dry_run);
    }
}