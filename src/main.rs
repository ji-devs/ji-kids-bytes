//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod settings;
mod logger;
mod server;
mod reject;
mod pages;
mod templates;
mod routes;
mod cors;
mod data;
mod loader;

use dotenv::dotenv;
use server::start_server;

#[tokio::main]
async fn main() {
    dotenv().ok();

    logger::init_logger();

    start_server().await;
}