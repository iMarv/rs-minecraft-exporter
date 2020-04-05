use player::gather_players;
use prometheus::Registry;
use prometheus_handler::{print_metrics, track_for_player};
use std::error;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate simple_logger;

mod player;
mod prometheus_handler;
mod stats;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    run().await?;

    Ok(())
}

async fn run() -> Result<()> {
    let players = gather_players(String::from("./test-data")).await?;
    // Each run needs its own registry as we just map the stats rather
    // than actually upping any counters
    let registry = Registry::new();

    for player in &players {
        track_for_player(&player, &registry);
    }

    print_metrics(registry);
    Ok(())
}
