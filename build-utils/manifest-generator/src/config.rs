use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "creation pack download", about = "A little util to download and re-parse creation packs")]
pub struct Config {
    /// Output dir
    #[structopt(short, long, default_value="D:\\Dropbox (Jewish Interactive)\\Ji Kids - Bytes - Media\\live-media\\manifests", parse(from_os_str))]
    pub local_output: PathBuf,
    
    /// Manifest Id 
    #[structopt(short, long, default_value="1kugXziYFFDwiJmIxQ-T_8cckOc4qSfHoFw9KoM56Hvs", parse(from_str))]
    pub manifest_list_id: String,

    /// dry run 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,

    /// how many per batch 
    #[structopt(long, parse(try_from_str), default_value = "10")]
    pub per_batch: usize,

    /// batch sleep time 
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub batch_sleep_time: u64,

    /// only locked 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub only_locked: bool,
}
