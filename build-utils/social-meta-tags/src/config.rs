use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "creation pack download", about = "A little util to download and re-parse creation packs")]
pub struct Config {
    /// Manifests dir 
    #[structopt(short, long, default_value="D:\\Dropbox (Jewish Interactive)\\Ji Kids - Bytes - Media\\live-media\\manifests", parse(from_os_str))]
    pub manifest_dir: PathBuf,

    /// html dir 
    #[structopt(short, long, default_value="../../dist", parse(from_os_str))]
    pub html_dir: PathBuf,
    
    /// Base title 
    #[structopt(long, default_value="JiKids Bytes", parse(from_str))]
    pub base_title: String,

    /// Base url 
    #[structopt(long, default_value="https://bytes.jikids.org", parse(from_str))]
    pub base_url: String,

    /// Base media url 
    #[structopt(long, default_value="https://storage.googleapis.com/bytes-ji-kids-eu", parse(from_str))]
    pub base_media_url: String,

    /// default description 
    #[structopt(long, default_value="A taste of Jewish learning for your kids for an hour a day!", parse(from_str))]
    pub default_description: String,

    /// default image 
    #[structopt(long, default_value="social/ji-bytes.png", parse(from_str))]
    pub default_image: String,

    /// dry run 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,
}
