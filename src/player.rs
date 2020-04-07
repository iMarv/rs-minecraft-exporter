use crate::stats::{NbtStats, Stats};
use crate::Result;
use fs::{DirEntry, File};
use nbt;
use serde::Deserialize;
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::sync::Mutex;

lazy_static! {
    static ref PLAYER_NAMES: Mutex<HashMap<String, String>> = { Mutex::new(HashMap::new()) };
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub stats: Stats,
    pub nbt_stats: NbtStats,
}

impl Player {
    pub async fn from_uuid(uuid: String, stats_path: &PathBuf, nbt_path: &PathBuf) -> Result<Self> {
        let name = get_player_name(&uuid).await?;
        let file_name = format!("{}.json", uuid);
        let path = stats_path.join(file_name);

        let stats = fs::read_to_string(path)?;
        let stats = Stats::from(stats)?;

        let nbt_stats = File::open(nbt_path)?;
        let nbt_stats: NbtStats = nbt::de::from_gzip_reader(nbt_stats)?;

        Ok(Self {
            name,
            stats,
            nbt_stats,
        })
    }
}

#[derive(Debug, Deserialize)]
struct NameResponse {
    name: String,
    #[serde(alias = "changedToAt")]
    changed_to_at: Option<usize>,
}

async fn get_player_name(uuid: &String) -> Result<String> {
    let mut names = PLAYER_NAMES.lock().await;

    if let Some(name) = names.get(uuid) {
        trace!("Got name from cache for {}", uuid);
        Ok(name.clone())
    } else if let Ok(name) = fetch_player_name(uuid).await {
        info!("Fetched name `{}` from api for id {}", name, uuid);
        names.insert(uuid.clone(), name.clone());
        Ok(name)
    } else {
        error!("No name found for UUID {}", uuid);
        Err("Name not found")?
    }
}

async fn fetch_player_name(uuid: &String) -> Result<String> {
    trace!("Fetching name from API for {}", uuid);

    let url = format!(
        "https://api.mojang.com/user/profiles/{}/names",
        uuid.replace('-', "")
    );

    match reqwest::get(&url)
        .await?
        .json::<Vec<NameResponse>>()
        .await?
        .last()
        .map(|res| res.name.clone())
    {
        Some(name) => Ok(name),
        None => Err("Not able to fetch name")?,
    }
}

pub async fn gather_players(base_path: &Path) -> Result<Vec<Player>> {
    let playerdata_path = base_path.join(Path::new("playerdata"));
    let stats_path = base_path.join(Path::new("stats"));

    let playerdata = fs::read_dir(playerdata_path)?;
    let mut result: Vec<Player> = vec![];

    for entry in playerdata {
        let nbt_file: DirEntry = entry?;
        let entry = &nbt_file.file_name();
        let entry = Path::new(entry).file_stem();

        if let Some(entry) = entry.and_then(|e| e.to_str()) {
            let p: Result<Player> =
                Player::from_uuid(String::from(entry), &stats_path, &nbt_file.path()).await;

            match p {
                Ok(player) => {
                    result.push(player);
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }

    Ok(result)
}
