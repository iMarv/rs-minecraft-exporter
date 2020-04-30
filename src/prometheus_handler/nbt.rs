use crate::player::Player;
use crate::prometheus_handler::stat_cache::STAT_CACHE;
use crate::Result;

macro_rules! local_register_gauge {
    // Multiple stats at once
    ($player:expr,
        $( [$s_name:expr, $help:expr, $val:expr] )+
    ) => {
        $(
            // Matcher for self, single stat version
            local_register_gauge!($s_name, $help, $val, $player);
        )+
    };
    // One stat
    ( $s_name:expr, $help:expr, $val:expr, $player:expr)  => {
        STAT_CACHE.set_gauge(
            $player,
            &String::from($s_name),
            &String::from($help),
            $val
        ).await?;
    };
}

pub async fn register_nbt_stats(player: &Player) -> Result<()> {
    local_register_gauge!(
        player,
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

    Ok(())
}
