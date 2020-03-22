use std::path::PathBuf;
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(name = "creation pack download", about = "A little util to download and re-parse creation packs")]
pub struct Config {
    /// Output dir
    #[structopt(short, long, default_value="D:\\Dropbox (Jewish Interactive)\\Daily_Bytes\\live-media\\manifests", parse(from_os_str))]
    pub local_output: PathBuf,
    
    /// Manifest Id 
    #[structopt(short, long, default_value="1kugXziYFFDwiJmIxQ-T_8cckOc4qSfHoFw9KoM56Hvs", parse(from_str))]
    pub manifest_list_id: String,

    /// dry run 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,
}

pub const MAX_SIMULTANEOUS:usize = 20;