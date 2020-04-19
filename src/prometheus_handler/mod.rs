use self::nbt::register_nbt_stat;
use self::playerstats::register_playerstats;
use crate::player::Player;
use prometheus::Registry;

mod nbt;
mod playerstats;

pub fn track_for_player(player: &Player, registry: &Registry) {
    register_playerstats(player, registry);
    register_nbt_stat(player, registry);
}
