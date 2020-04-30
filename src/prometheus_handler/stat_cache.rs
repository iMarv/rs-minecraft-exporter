use crate::{player::Player, stats::StatCategory};
use prometheus::{default_registry, Counter, Gauge, Registry};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type CounterCache = HashMap<String, Counter>;
type GaugeCache = HashMap<String, Gauge>;

lazy_static! {
    static ref STAT_CACHE: StatCache = StatCache::new();
}

struct StatCache {
    counter_cache: Arc<Mutex<CounterCache>>,
    gauge_cache: Arc<Mutex<GaugeCache>>,
    registry: &'static Registry,
}

impl Default for StatCache {
    fn default() -> Self {
        Self {
            counter_cache: Arc::new(Mutex::new(HashMap::new())),
            gauge_cache: Arc::new(Mutex::new(HashMap::new())),
            registry: default_registry(),
        }
    }
}

impl StatCache {
    pub fn new() -> Self {
        StatCache::default()
    }

    pub async fn set_counter(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
        value: f64,
    ) {
        let counter = self.get_counter(player, category, category_type).await;
        counter.inc_by(value - counter.get());
    }

    pub async fn set_gauge(
        &self,
        player: &Player,
        category_type: &String,
        category_help: &String,
        value: f64,
    ) {
        self.get_gauge(player, category_type, category_help)
            .await
            .set(value);
    }

    async fn get_gauge(
        &self,
        player: &Player,
        category_name: &String,
        category_help: &String,
    ) -> Gauge {
        let id = gauge_id(player, category_name);
        let mut gauge_cache = self.gauge_cache.lock().await;

        if !gauge_cache.contains_key(&id) {
            let labels: HashMap<&str, &String> = labels!(
                "player" => &player.name,
            );

            let gauge = Gauge::with_opts(opts!(category_name, category_help, labels)).unwrap();

            self.registry
                .register(Box::new(gauge.clone()))
                .expect("Stat got registered twice. This should not happen.");

            gauge_cache.insert(id.clone(), gauge);
        }

        gauge_cache.get(&id).unwrap().clone()
    }

    async fn get_counter(
        &self,
        player: &Player,
        category: &StatCategory,
        category_type: &String,
    ) -> Counter {
        let id = counter_id(player, category, category_type);
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

fn counter_id(player: &Player, category: &StatCategory, category_type: &String) -> String {
    format!("{}_{}_{}", &player.uuid, category, category_type)
}

fn gauge_id(player: &Player, category_name: &String) -> String {
    format!("{}_{}", &player.uuid, category_name)
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

    macro_rules! mock_gauge_cache {
        () => {
            mock_gauge_cache!(
                &mock_player!(999999),
                &String::from("some_gauge_name"),
                1337.0
            )
        };
        ($player:expr, $category_name:expr, $value:expr) => {{
            let id = gauge_id($player, $category_name);
            let gauge = Gauge::with_opts(opts!(
                $category_name.clone(),
                format!("help: {}", $category_name)
            ))
            .unwrap();
            gauge.set($value);

            let mut c = HashMap::new();
            c.insert(id, gauge);

            Arc::new(Mutex::new(c))
        }};
    }

    macro_rules! mock_counter_cache {
        () => {
            mock_counter_cache!(
                &mock_player!(999999),
                &StatCategory::KilledBy,
                &String::from("some_type"),
                10.0
            )
        };
        ($player:expr, $stat_category:expr, $category_type:expr, $value:expr) => {{
            let id = counter_id($player, $stat_category, $category_type);
            let counter = Counter::with_opts(opts!("Testi", "Help")).unwrap();
            counter.inc_by($value);

            let mut c = HashMap::new();
            c.insert(id, counter);

            Arc::new(Mutex::new(c))
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

    mod set_gauge {
        use super::*;

        #[tokio::test]
        async fn should_update_existing_stat() {
            let player = mock_player!(1);
            let category_name = String::from("gauge_name");
            let category_help = String::from("minimal amount of xp");
            let value = 2.0;

            let cache = StatCache {
                gauge_cache: mock_gauge_cache!(&player, &category_name, value),
                counter_cache: mock_counter_cache!(),
                registry: default_registry(),
            };

            let value = 5.0;
            cache
                .set_gauge(&player, &category_name, &category_help, value)
                .await;

            let actual = cache
                .get_gauge(&player, &category_name, &category_help)
                .await;

            assert_eq!(actual.get(), value);
        }

        #[tokio::test]
        async fn should_insert_new_stat() {
            let player = mock_player!(1);
            let category_name = String::from("gauge_name");
            let category_help = String::from("minimal amount of xp");
            let value = 2.0;

            let cache = StatCache {
                gauge_cache: mock_gauge_cache!(),
                counter_cache: mock_counter_cache!(),
                registry: default_registry(),
            };

            cache
                .set_gauge(&player, &category_name, &category_help, value)
                .await;

            let actual = cache
                .get_gauge(&player, &category_name, &category_help)
                .await;

            assert_eq!(actual.get(), value);
        }
    }

    mod set_counter {
        use super::*;

        #[tokio::test]
        async fn should_update_existing_stat() {
            let player = mock_player!(1);
            let category = StatCategory::Crafted;
            let category_type = String::from("minecraft:test1");
            let value = 2.0;

            let cache = StatCache {
                gauge_cache: mock_gauge_cache!(),
                counter_cache: mock_counter_cache!(&player, &category, &category_type, value),
                registry: default_registry(),
            };

            let value = 5.0;
            cache
                .set_counter(&player, &category, &category_type, value)
                .await;

            let actual = cache.get_counter(&player, &category, &category_type).await;

            assert_eq!(actual.get(), value);
        }

        #[tokio::test]
        async fn should_insert_new_stat() {
            let cache = StatCache {
                gauge_cache: mock_gauge_cache!(),
                counter_cache: mock_counter_cache!(),
                registry: default_registry(),
            };

            let player = &mock_player!(1);
            let category = &StatCategory::Crafted;
            let category_type = &String::from("minecraft:test2");
            let value = 2.0;

            cache
                .set_counter(player, category, category_type, value)
                .await;

            let actual = cache.get_counter(player, category, category_type).await;

            assert_eq!(actual.get(), value);
        }
    }
}
