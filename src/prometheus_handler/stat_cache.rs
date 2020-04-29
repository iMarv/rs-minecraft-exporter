use crate::{player::Player, stats::StatCategory};
use prometheus::{default_registry, Counter, Registry};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type Uuid = String;
type CategoryTypeCache = HashMap<String, Counter>;
type CategoryCache = HashMap<StatCategory, CategoryTypeCache>;
type PlayerCache = HashMap<Uuid, CategoryCache>;

struct StatCache {
    player_cache: Arc<Mutex<PlayerCache>>,
    registry: &'static Registry,
}

impl Default for StatCache {
    fn default() -> Self {
        Self {
            player_cache: Arc::new(Mutex::new(HashMap::new())),
            registry: default_registry(),
        }
    }
}

impl StatCache {
    pub fn new() -> Self {
        StatCache::default()
    }

    pub async fn set_stat(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
        value: f64,
    ) {
        let counter = self.get_counter(player, category, category_type).await;
        counter.inc_by(value - counter.get());
    }

    async fn get_category_cache(&self, player: &Player) -> CategoryCache {
        let mut player_cache = self.player_cache.lock().await;

        if !player_cache.contains_key(&player.uuid) {
            player_cache.insert(player.uuid.clone(), HashMap::new());
        }

        player_cache.get(&player.uuid).unwrap().clone()
    }

    async fn get_type_cache(&self, player: &Player, category: &StatCategory) -> CategoryTypeCache {
        let mut category_cache = self.get_category_cache(player).await;

        if !category_cache.contains_key(category) {
            category_cache.insert(category.clone(), HashMap::new());
        }

        category_cache.get(category).unwrap().clone()
    }

    pub async fn get_counter(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
    ) -> Counter {
        let category_type_cache = self.get_type_cache(player, category).await;

        if !category_type_cache.contains_key(category_type) {
            todo!();
        }

        category_type_cache.get(category_type).unwrap().clone()
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
                player_cache: Arc::new(Mutex::new(player_cache)),
                registry: default_registry(),
            }
        }};
    }

    mod set_stat {
        use super::*;

        #[tokio::test]
        async fn should_update_existing_stat() {
            let player = &mock_player!(1);
            let category = &StatCategory::Crafted;
            let category_type = &String::from("minecraft:testo");
            let value = 2.0;

            let cache = setup_cache!(1, StatCategory::Crafted, category_type, value);

            let value = 5.0;
            cache.set_stat(player, category, category_type, value).await;

            let actual = cache.get_counter(player, category, category_type).await;

            assert_eq!(actual.get(), value);
        }

        #[tokio::test]
        async fn should_insert_new_stat() {
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

            cache.set_stat(player, category, category_type, value).await;

            let actual = cache.get_counter(player, category, category_type).await;

            assert_eq!(actual.get(), value);
        }
    }
}
