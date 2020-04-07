use crate::{player::Player, stats::StatCategory};
use prometheus::{Counter, Registry};
use std::collections::HashMap;

macro_rules! local_register_counter {
    // Multiple stats at once
    ($reg:expr, $p_name:expr,
        $( [$s_name:expr, $help:expr, $val:expr $(, $s_type:expr)?] )+
    ) => {
        $(
            // Matcher for self, single stat version
            local_register_counter!($reg, $s_name, $help, $val, $p_name $(, $s_type:expr)?);
        )+
    };
    // One stat
    ( $reg:expr, $s_name:expr, $help:expr, $val:expr, $p_name:expr $(, $s_type:expr)? )  => {
        let labels: HashMap<&str, &String> = labels!(
            "player" => $p_name,
            $(
                "type" => $s_type,
            )?
        );

        let counter = Counter::with_opts(opts!(&String::from($s_name), &String::from($help), labels)).unwrap();
        counter.inc_by($val);

        $reg.register(Box::new(counter)).unwrap();
    };
}

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
    local_register_counter!(
        registry,
        &player.name,
        [
            // XpTotal
            "mc_xp_total",
            "total collceted xp",
            player.nbt_stats.xp_total
        ]
        [
            // XpLevel
            "mc_xp_level",
            "current player level",
            player.nbt_stats.xp_level
        ]
        [
            // Score
            "mc_score",
            "current player score",
            player.nbt_stats.score
        ]
        [
            // Health
            "mc_health",
            "current player health",
            player.nbt_stats.health
        ]
        [
            // foodLevel
            "mc_food_level",
            "current player food level",
            player.nbt_stats.food_level
        ]
    );
}

fn track_playerstat(player: &Player, stat: StatCategory, registry: &Registry) {
    let stat_str = remove_prefix(&stat.to_string());
    let name = format!("mc_{}", stat_str);
    let help = format!("collected stats for category `{}`", stat_str);

    if let Some(stats) = player.stats.get_stat(stat) {
        for (key, value) in stats.iter() {
            let value = value.as_f64().expect("Property value not a number");

            local_register_counter!(registry, &name, &help, value, &player.name, &key);
        }
    } else {
        trace!(
            "Missing category `{}` for player `{}`",
            stat_str,
            player.name
        );
    }
}

fn remove_prefix(property: &String) -> String {
    property[10..].to_string()
}
