use serde::Deserialize;
use serde_json::Result;
use serde_json::{Map, Value};
use std::cmp::Eq;
use std::fmt::Display;

pub const STAT_CATEGORIES: [StatCategory; 9] = [
    StatCategory::Mined,
    StatCategory::Crafted,
    StatCategory::Broken,
    StatCategory::Custom,
    StatCategory::PickedUp,
    StatCategory::KilledBy,
    StatCategory::Used,
    StatCategory::Dropped,
    StatCategory::Killed,
];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum StatCategory {
    Mined,
    Crafted,
    Broken,
    Custom,
    PickedUp,
    KilledBy,
    Used,
    Dropped,
    Killed,
}

impl Display for StatCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            StatCategory::Mined => "mined",
            StatCategory::Crafted => "crafted",
            StatCategory::Broken => "broken",
            StatCategory::Custom => "custom",
            StatCategory::PickedUp => "picked_up",
            StatCategory::KilledBy => "killed_by",
            StatCategory::Used => "used",
            StatCategory::Dropped => "dropped",
            StatCategory::Killed => "killed",
        };

        write!(f, "minecraft:{}", name)
    }
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    stats: Value,
}

impl Stats {
    pub fn from(data: String) -> Result<Stats> {
        let stats: Stats = serde_json::from_str(&data)?;

        Ok(stats)
    }

    pub fn get_stat(&self, category: &StatCategory) -> Option<&Map<String, Value>> {
        self.stats[category.to_string()].as_object()
    }
}

#[macro_export]
macro_rules! mock_stats {
    () => {
        crate::stats::Stats::from(String::from(
            "{
               \"stats\": {
                   \"minecraft:testo\": 42
               }
            }",
        ))
        .unwrap()
    };
}

#[derive(Debug, Deserialize)]
pub struct NbtStats {
    #[serde(alias = "XpTotal")]
    pub xp_total: f64,
    #[serde(alias = "XpLevel")]
    pub xp_level: f64,
    #[serde(alias = "Score")]
    pub score: f64,
    #[serde(alias = "Health")]
    pub health: f64,
    #[serde(alias = "foodLevel")]
    pub food_level: f64,
}

#[macro_export]
macro_rules! mock_nbt {
    () => {
        crate::stats::NbtStats {
            xp_total: 100.0,
            xp_level: 101.0,
            score: 102.0,
            health: 10.0,
            food_level: 10.0,
        }
    };
}
