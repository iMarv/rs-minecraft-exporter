use crate::{player::Player, stats::STAT_CATEGORIES};
use prometheus::{Counter, Registry};
use std::collections::HashMap;
use tokio::sync::Mutex;

type PlayerName = String;

lazy_static! {
    static ref PLAYER_COUNTERS: Mutex<HashMap<PlayerName, HashMap<String, Counter>>> =
        { Mutex::new(HashMap::new()) };
}

async fn update_playerstats() {
    todo!();
}

pub fn register_playerstats(player: &Player, registry: &Registry) {
    for stat in STAT_CATEGORIES.iter() {
        let stat_str = {
            let s = &stat.to_string();
            s[10..].to_string()
        };

        let name = format!("mc_{}", stat_str);
        let help = format!("collected stats for category `{}`", stat_str);

        if let Some(stats) = player.stats.get_stat(stat) {
            for (key, value) in stats.iter() {
                let value = value.as_f64().expect("Property value not a number");

                let labels: HashMap<&str, &String> = labels!(
                    "player" => &player.name,
                    "type" => key,
                );

                let counter =
                    Counter::with_opts(opts!(&String::from(&name), &String::from(&help), labels))
                        .unwrap();
                counter.inc_by(value);

                registry.register(Box::new(counter)).unwrap();
            }
        } else {
            trace!(
                "Missing category `{}` for player `{}`",
                stat_str,
                player.name
            );
        }
    }
}
