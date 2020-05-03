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
    pub uuid: String,
    pub name: String,
    pub stats: Stats,
    pub nbt_stats: NbtStats,
}

impl Player {
    pub async fn from_uuid(uuid: String, stats_path: &PathBuf, nbt_path: &PathBuf) -> Result<Self> {
        let name = get_player_name(&uuid).await?;
        let file_name = format!("{}.json", uuid);
        let path = stats_path.join(file_name);

        let stats = {
            let s = fs::read_to_string(path)?;
            Stats::from(s)?
        };

        let nbt_stats = {
            let n = File::open(nbt_path)?;
            nbt::de::from_gzip_reader(n)?
        };

        Ok(Self {
            uuid,
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
    } else if let Ok(name) = fetch_from_mojang(uuid).await {
        info!("Fetched name `{}` from api for id {}", name, uuid);
        names.insert(uuid.clone(), name.clone());
        Ok(name)
    } else {
        error!("No name found for UUID {}", uuid);
        Err("Name not found")?
    }
}

async fn fetch_from_mojang(uuid: &String) -> Result<String> {
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
    let stats_path = base_path.join(Path::new("stats"));

    if !stats_path.exists() {
        return Err("Target directory does not contain a stats folder".into());
    }

    let playerdata = {
        let p = base_path.join(Path::new("playerdata"));
        if !p.exists() {
            return Err("Target directory does not contain a playerdata folder".into());
        }

        fs::read_dir(p)?
    };

    let mut result: Vec<Player> = vec![];

    for entry in playerdata {
        let nbt_file: DirEntry = entry?;
        let entry = &nbt_file.file_name();
        let entry = Path::new(entry).file_stem();

        if let Some(entry) = entry.and_then(|e| e.to_str()) {
            let player: Result<Player> =
                Player::from_uuid(String::from(entry), &stats_path, &nbt_file.path()).await;

            match player {
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

#[macro_export]
macro_rules! mock_player {
    ($id:expr) => {
        Player {
            name: format!("name-{}", $id),
            nbt_stats: crate::mock_nbt!(),
            uuid: format!("{}", $id),
            stats: crate::mock_stats!(),
        }
    };
}
