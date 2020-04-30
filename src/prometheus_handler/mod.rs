use crate::player::Player;
use crate::prometheus_handler::nbt::register_nbt_stats;
use playerstats::register_playerstats;

mod nbt;
mod playerstats;
mod stat_cache;

pub async fn track_for_player(player: &Player) {
    register_playerstats(player).await;
    register_nbt_stats(player).await;
}
