use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "creation pack download", about = "A little util to download and re-parse creation packs")]
pub struct Config {
    /// Output dir
    
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

    /// list top-level items only 
    #[structopt(long)]
    pub list_topic_meta_only: bool,

    /// list top-level items only 
    #[structopt(long)]
    pub list_series_order_only: bool,

    #[structopt(long)]
    pub no_topics: bool,

    /// Ignore the sync settings in the sheet and sync anyway
    #[structopt(long)]
    pub sync_all: bool,
}

impl Config {
    pub fn sanitize(mut self) -> Self {
        if self.list_topic_meta_only || self.list_series_order_only {
            self.dry_run = true;
        }
        self
    }

    pub fn list_only(&self) -> bool {
        self.list_topic_meta_only || self.list_series_order_only
    }
}
