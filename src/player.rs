use crate::stats::Stats;
use crate::Result;
use fs::DirEntry;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::{
    fs,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref PLAYER_NAMES: Mutex<HashMap<String, String>> = { Mutex::new(HashMap::new()) };
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub stats: Stats,
}

impl Player {
    pub async fn from_uuid(uuid: String, stats_path: &PathBuf) -> Result<Self> {
        let name = get_player_name(&uuid).await?;
        let file_name = format!("{}.json", uuid);
        let path = stats_path.join(file_name);

        let stats = fs::read_to_string(path)?;
        let stats = Stats::from(stats)?;

        Ok(Self { name, stats })
    }
}

#[derive(Debug, Deserialize)]
struct NameResponse {
    name: String,
    #[serde(alias = "changedToAt")]
    changed_to_at: Option<usize>,
}

async fn get_player_name(uuid: &String) -> Result<String> {
    if let Ok(names) = PLAYER_NAMES.lock() {
        let mut names: MutexGuard<HashMap<String, String>> = names;

        if let Some(name) = names.get(uuid) {
            trace!("Got name from cache for {}", uuid);
            Ok(name.clone())
        } else if let Ok(name) = fetch_player_name(uuid).await {
            names.insert(uuid.clone(), name.clone());
            Ok(name)
        } else {
            error!("No name found for UUID {}", uuid);
            Err("Name not found")?
        }
    } else {
        warn!("Name cache locked, fetching from API");
        fetch_player_name(uuid).await
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

pub async fn gather_players(base_dir: String) -> Result<Vec<Player>> {
    let base_path = Path::new(&base_dir);
    let playerdata_path = base_path.join(Path::new("playerdata"));
    let stats_path = base_path.join(Path::new("stats"));

    let playerdata = fs::read_dir(playerdata_path)?;
    let mut result: Vec<Player> = vec![];

    for entry in playerdata {
        let entry: DirEntry = entry?;
        let entry = &entry.file_name();
        let entry = Path::new(entry).file_stem();

        if let Some(entry) = entry.and_then(|e| e.to_str()) {
            let p: Result<Player> = Player::from_uuid(String::from(entry), &stats_path).await;

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
