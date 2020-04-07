use crate::{player::Player, stats::StatCategory};
use prometheus::{Counter, Registry};
use std::collections::HashMap;

pub fn track_for_player(player: &Player, registry: &Registry) {
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
        track_playerstat(player, *metric, registry);
    }

    track_nbt_stat(player, registry);
}

fn track_nbt_stat(player: &Player, registry: &Registry) {
    // XpTotal
    register_stat(
        registry,
        &String::from("mc_xp_total"),
        &String::from("total collected xp"),
        player.nbt_stats.xp_total,
        &player.name,
        None,
    );
    // XpLevel
    register_stat(
        registry,
        &String::from("mc_xp_level"),
        &String::from("current player level"),
        player.nbt_stats.xp_level,
        &player.name,
        None,
    );
    // Score
    register_stat(
        registry,
        &String::from("mc_score"),
        &String::from("current player score"),
        player.nbt_stats.score,
        &player.name,
        None,
    );
    // Health
    register_stat(
        registry,
        &String::from("mc_health"),
        &String::from("current player health"),
        player.nbt_stats.health,
        &player.name,
        None,
    );
    // foodLevel
    register_stat(
        registry,
        &String::from("mc_food_level"),
        &String::from("current player food level"),
        player.nbt_stats.health,
        &player.name,
        None,
    );
}

fn track_playerstat(player: &Player, stat: StatCategory, registry: &Registry) {
    let stat_str = remove_prefix(&stat.to_string());
    let name = format!("mc_{}", stat_str);
    let help = format!("collected stats for category `{}`", stat_str);

    if let Some(stats) = player.stats.get_stat(stat) {
        for (key, value) in stats.iter() {
            let value = value.as_f64().expect("Property value not a number");

            register_stat(registry, &name, &help, value, &player.name, Some(&key));
        }
    } else {
        trace!(
            "Missing category `{}` for player `{}`",
            stat_str,
            player.name
        );
    }
}

fn register_stat(
    registry: &Registry,
    stat_name: &String,
    help: &String,
    value: f64,
    player_name: &String,
    stat_type: Option<&String>,
) {
    let mut labels: HashMap<&str, &String> = labels!(
        "player" => player_name,
    );

    if let Some(stat_type) = stat_type {
        labels.insert("type", stat_type);
    }

    let counter = Counter::with_opts(opts!(stat_name, help, labels)).unwrap();
    counter.inc_by(value);

    registry.register(Box::new(counter.clone())).unwrap();
}

fn remove_prefix(property: &String) -> String {
    property[10..].to_string()
}
