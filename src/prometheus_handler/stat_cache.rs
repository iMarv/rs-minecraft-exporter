use crate::{player::Player, stats::StatCategory};
use prometheus::{default_registry, Counter, Registry};
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
    static ref PLAYER_COUNTERS: Mutex<PlayerCache> = { Mutex::new(HashMap::new()) };
}

type Uuid = String;
type CategoryTypeCache = HashMap<String, Counter>;
type CategoryCache = HashMap<StatCategory, CategoryTypeCache>;
type PlayerCache = HashMap<Uuid, CategoryCache>;

pub struct StatCache {
    player_cache: PlayerCache,
    registry: &'static Registry,
}

impl StatCache {
    pub fn set_stat(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
        value: f64,
    ) {
        if let Some(mut counter) = self.get_counter(player, category, category_type) {
            counter.inc_by(value - counter.get());
        };
    }
}

// Non-mutable
impl StatCache {
    pub fn new() -> Self {
        Self {
            player_cache: HashMap::new(),
            registry: default_registry(),
        }
    }

    fn get_category_cache(&self, player: &Player) -> Option<&CategoryCache> {
        self.player_cache.get(&player.uuid)
    }

    fn get_type_cache(
        &self,
        player: &Player,
        category: &StatCategory,
    ) -> Option<&CategoryTypeCache> {
        self.get_category_cache(player)
            .and_then(|cache| cache.get(category))
    }

    pub fn get_counter(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
    ) -> Option<&Counter> {
        self.get_type_cache(player, category)
            .and_then(|cache| cache.get(category_type))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_player;
    use prometheus::Registry;

    macro_rules! setup_cache {
        ($player_id:expr, $stat_category:expr, $category_type:expr, $value:expr) => {{
            let category_type_cache = {
                let counter = Counter::with_opts(opts!("Testi", "Help")).unwrap();
                counter.inc_by($value);

                let mut c: CategoryTypeCache = HashMap::new();
                c.insert(String::from($category_type), counter);

                c
            };

            let category_cache: CategoryCache = {
                let mut c: CategoryCache = HashMap::new();
                c.insert($stat_category, category_type_cache);

                c
            };

            let player_cache: PlayerCache = {
                let player = mock_player!($player_id);
                let mut c: PlayerCache = HashMap::new();
                c.insert(player.uuid, category_cache);
                c
            };

            StatCache {
                player_cache,
                registry: default_registry(),
            }
        }};
    }

    mod set_stat {
        use super::*;

        #[test]
        fn should_update_existing_stat() {
            let player = &mock_player!(1);
            let category = &StatCategory::Crafted;
            let category_type = &String::from("minecraft:testo");
            let value = 2.0;

            let cache = setup_cache!(1, StatCategory::Crafted, category_type, value);

            let value = 5.0;
            cache.set_stat(player, category, category_type, value);

            let actual = cache.get_counter(player, category, category_type);

            assert!(actual.is_some());
            assert_eq!(actual.unwrap().get(), value);
        }

        #[test]
        fn should_insert_new_stat() {
            let cache = setup_cache!(
                3,
                StatCategory::Broken,
                &String::from("minecraft:testi"),
                67.0
            );
            let player = &mock_player!(1);
            let category = &StatCategory::Crafted;
            let category_type = &String::from("minecraft:testo");
            let value = 2.0;

            cache.set_stat(player, category, category_type, value);

            let actual = cache.get_counter(player, category, category_type);

            assert!(actual.is_some());
            assert_eq!(actual.unwrap().get(), value);
        }
    }
}
