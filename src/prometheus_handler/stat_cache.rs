use crate::{player::Player, stats::StatCategory};
use prometheus::{default_registry, Counter, Registry};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type CounterCache = HashMap<String, Counter>;

lazy_static! {
    static ref STAT_CACHE: StatCache = StatCache::new();
}

struct StatCache {
    counter_cache: Arc<Mutex<CounterCache>>,
    registry: &'static Registry,
}

impl Default for StatCache {
    fn default() -> Self {
        Self {
            counter_cache: Arc::new(Mutex::new(HashMap::new())),
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

    pub async fn get_counter(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
    ) -> Counter {
        let id = unique_stat_id(player, category, category_type);
        let mut counter_cache = self.counter_cache.lock().await;

        if !counter_cache.contains_key(&id) {
            let labels: HashMap<&str, &String> = labels!(
                "player" => &player.name,
                "type" => category_type,
            );

            let (category_name, category_help) = get_category_metadata(category);

            let counter = Counter::with_opts(opts!(
                &String::from(&category_name),
                &String::from(&category_help),
                labels
            ))
            .unwrap();

            self.registry
                .register(Box::new(counter.clone()))
                .expect("Stat got registered twice. This should not happen.");

            counter_cache.insert(id.clone(), counter);
        }

        counter_cache.get(&id).unwrap().clone()
    }
}

fn unique_stat_id(player: &Player, category: &StatCategory, category_type: &String) -> String {
    format!("{}_{}_{}", &player.uuid, category, category_type)
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
        ($player:expr, $stat_category:expr, $category_type:expr, $value:expr) => {{
            let counter_cache = {
                let id = unique_stat_id($player, $stat_category, $category_type);
                let counter = Counter::with_opts(opts!("Testi", "Help")).unwrap();
                counter.inc_by($value);

                let mut c = HashMap::new();
                c.insert(id, counter);

                c
            };

            StatCache {
                counter_cache: Arc::new(Mutex::new(counter_cache)),
                registry: default_registry(),
            }
        }};
    }

    mod get_category_metadata {
        use super::*;

        #[test]
        fn should_remove_prefix() {
            let category = StatCategory::Broken;

            let (actual, _) = get_category_metadata(&category);
            let expected = String::from("mc_broken");

            assert_eq!(actual, expected);
        }
    }

    mod set_stat {
        use super::*;

        #[tokio::test]
        async fn should_update_existing_stat() {
            let player = &mock_player!(1);
            let category = &StatCategory::Crafted;
            let category_type = &String::from("minecraft:test1");
            let value = 2.0;

            let cache = setup_cache!(&player, category, category_type, value);

            let value = 5.0;
            cache.set_stat(player, category, category_type, value).await;

            let actual = cache.get_counter(player, category, category_type).await;

            assert_eq!(actual.get(), value);
        }

        #[tokio::test]
        async fn should_insert_new_stat() {
            let cache = setup_cache!(
                &mock_player!(3),
                &StatCategory::Broken,
                &String::from("minecraft:testi"),
                67.0
            );
            let player = &mock_player!(1);
            let category = &StatCategory::Crafted;
            let category_type = &String::from("minecraft:test2");
            let value = 2.0;

            cache.set_stat(player, category, category_type, value).await;

            let actual = cache.get_counter(player, category, category_type).await;

            assert_eq!(actual.get(), value);
        }
    }
}
