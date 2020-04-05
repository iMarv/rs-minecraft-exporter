use player_stats::StatCategory;
use prometheus_handler::{print_metrics, track_metric};
use std::fs;

mod player_stats;
mod prometheus_handler;

fn main() {
    let data = fs::read_to_string("./test-data/data.json").unwrap();
    let data = player_stats::Stats::from(data).unwrap();
    let player = player_stats::Player {
        name: String::from("Testo"),
        stats: data,
    };

    for metric in [
        StatCategory::Broken,
        StatCategory::Crafted,
        StatCategory::Custom,
        StatCategory::Dropped,
        StatCategory::Killed,
        StatCategory::KilledBy,
        StatCategory::Mined,
        StatCategory::PickedUp,
        StatCategory::Used,
    ]
    .iter()
    {
        track_metric(&player, *metric);
    }

    print_metrics();
}
