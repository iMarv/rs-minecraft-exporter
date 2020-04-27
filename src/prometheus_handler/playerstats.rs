use crate::{
    player::Player,
    stats::{StatCategory, STAT_CATEGORIES},
};
use prometheus::{Counter, Registry};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::Mutex;

type Uuid = String;
type StatCache = HashMap<String, Counter>;
type CategoryCache = HashMap<StatCategory, StatCache>;
type PlayerCache = HashMap<Uuid, CategoryCache>;

lazy_static! {
    static ref PLAYER_COUNTERS: Mutex<PlayerCache> = { Mutex::new(HashMap::new()) };
}

pub async fn process_player(player: &Player, registry: &Registry) {
    if let Some(mut counters) = PLAYER_COUNTERS.lock().await.get(&player.uuid) {
        update_playerstats(player, registry, counters).await;
    } else {
        register_playerstats(player, registry).await;
    }
}

async fn update_playerstats(
    player: &Player,
    registry: &Registry,
    counters: &HashMap<String, Counter>,
) {
    todo!();
}

async fn register_playerstats(player: &Player, registry: &Registry) {
    for category in STAT_CATEGORIES.iter() {
        let mut counters: StatCache = HashMap::new();

        let (category_name, category_help) = get_category_metadata(category);

        if let Some(stats) = player.stats.get_stat(category) {
            for (key, value) in stats.iter() {
                let value = value.as_f64().expect("Property value not a number");

                let labels: HashMap<&str, &String> = labels!(
                    "player" => &player.name,
                    "type" => key,
                );

                let counter = Counter::with_opts(opts!(
                    &String::from(&category_name),
                    &String::from(&category_help),
                    labels
                ))
                .unwrap();

                counter.inc_by(value);

                registry.register(Box::new(counter)).unwrap();

                counters.insert(key.clone(), counter);
            }
        }
    }
}

async fn register_stat_in_player(
    player: &Player,
    category: &StatCategory,
    category_type: &String,
    counter: Counter,
) {
    let stat_cache = {
        let mut stat_cache = HashMap::new();
        stat_cache.insert(category_type, counter);
        stat_cache
    };

    if let Some(mut player_cache) = PLAYER_COUNTERS.lock().await.get(&player.uuid) {
        // player in cache
    } else {
        // player not cached yet
    }
}

fn get_category_metadata(category: &StatCategory) -> (String, String) {
    let stat_str = {
        let s = &category.to_string();
        s[10..].to_string()
    };

    let name = format!("mc_{}", stat_str);
    let help = format!("collected stats for category `{}`", stat_str);

    (name, help)
}
