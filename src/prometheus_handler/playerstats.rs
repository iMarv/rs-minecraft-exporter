use crate::Result;
use crate::{player::Player, prometheus_handler::stat_cache::STAT_CACHE, stats::STAT_CATEGORIES};

pub async fn register_playerstats(player: &Player) -> Result<()> {
    for category in STAT_CATEGORIES.iter() {
        if let Some(stats) = player.stats.get_stat(category) {
            for (key, value) in stats.iter() {
                let value = value.as_f64().expect("Property value not a number");

                STAT_CACHE.set_counter(player, category, key, value).await?;
            }
        }
    }

    Ok(())
}
