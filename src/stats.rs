use serde::Deserialize;
use serde_json::Result;
use serde_json::{Map, Value};
use std::fmt::Display;

#[derive(Copy, Clone)]
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

    pub fn get_stat(&self, stat: StatCategory) -> Option<&Map<String, Value>> {
        self.stats[stat.to_string()].as_object()
    }
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