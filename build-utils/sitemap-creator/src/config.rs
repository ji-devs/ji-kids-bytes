use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "creation pack download", about = "A little util to download and re-parse creation packs")]
pub struct Config {
    /// Manifests dir 
    #[structopt(short, long, default_value="D:\\Dropbox (Jewish Interactive)\\Ji Kids - Bytes - Media\\live-media\\manifests", parse(from_os_str))]
    pub manifest_dir: PathBuf,

    /// sitemap output 
    #[structopt(short, long, default_value="../../dist/sitemap.xml", parse(from_os_str))]
    pub sitemap_file: PathBuf,
    
    /// Base url 
    #[structopt(long, default_value="https://bytes.jikids.org", parse(from_str))]
    pub base_url: String,

    /// dry run 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,
}
