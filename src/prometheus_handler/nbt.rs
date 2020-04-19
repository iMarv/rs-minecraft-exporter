use crate::player::Player;
use prometheus::{Gauge, Registry};
use std::collections::HashMap;
use tokio::sync::Mutex;

type PlayerName = String;

macro_rules! local_register_gauge {
    // Multiple stats at once
    ($reg:expr, $p_name:expr,
        $( [$s_name:expr, $help:expr, $val:expr] )+
    ) => {
        $(
            // Matcher for self, single stat version
            local_register_gauge!($reg, $s_name, $help, $val, $p_name);
        )+
    };
    // One stat
    ( $reg:expr, $s_name:expr, $help:expr, $val:expr, $p_name:expr)  => {
        let labels: HashMap<&str, &String> = labels!(
            "player" => $p_name,
        );

        let gauge = Gauge::with_opts(opts!(&String::from($s_name), &String::from($help), labels)).unwrap();
        gauge.set($val);

        $reg.register(Box::new(gauge)).unwrap();
    };
}

lazy_static! {
    static ref NBT_GAUGES: Mutex<HashMap<PlayerName, HashMap<String, Gauge>>> =
        { Mutex::new(HashMap::new()) };
}

pub fn update_nbt_stat() {
    todo!();
}

pub fn register_nbt_stat(player: &Player, registry: &Registry) {
    local_register_gauge!(
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
