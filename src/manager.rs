use std::{collections::HashMap, fs::File};

use rodio::OutputStreamHandle;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use crate::{audio::{Audio, PlayerInfo}, utils::IdPool};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerEntry {
    pub id: u32,
    pub info: PlayerInfo
}

impl PlayerEntry {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn info(&self) -> &PlayerInfo {
        &self.info
    }
}

#[derive(Default)]
pub struct PlayerManager {
    id_pool: IdPool,
    players: HashMap<u32, Audio>
}

impl PlayerManager {
    pub fn new() -> PlayerManager {
        PlayerManager {
            id_pool: IdPool::new(),
            players: HashMap::new()
        }
    }

    pub fn json(&self) -> String {
        let entries: Vec<PlayerEntry> = self.players
            .iter()
            .map(
                |(id, audio)| PlayerEntry {
                    id: *id,
                    info: audio.info().clone()
                }
            )
            .collect();

        serde_json::to_string(&entries).unwrap()
    }

    pub fn save(&mut self, filepath: &str) {
        if let Ok(file) = File::create(filepath) {
            let entries: Vec<PlayerEntry> = self.players
                .iter()
                .map(
                    |(id, audio)| PlayerEntry {
                        id: *id,
                        info: audio.info().clone()
                    }
                )
                .collect();

            serde_json::to_writer_pretty(file, &entries).unwrap();
        }
    }

    pub fn load(filepath: &str, handle: &OutputStreamHandle) -> PlayerManager {
        let mut manager = PlayerManager::default();

        if let Ok(file) = File::open(filepath) {
            let entries: Vec<PlayerEntry> = serde_json::from_reader(file).expect("Failed to deserialize AudioInfo JSON");

            for entry in entries {
                match Audio::from_info(entry.info, &handle) {
                    Ok(audio) => {
                        manager.players.insert(entry.id, audio);
                        manager.id_pool.reserve(entry.id);
                    },
                    Err(e) => {
                        error!("Failed to load Audio {}: {:?}", entry.id, e);
                    }
                }
            }
        } else {
            error!("Could not open '{}', starting with empty manager", filepath);
        }

        manager
    }

    pub fn get_player(&self, id: u32) -> Option<&Audio> {
        self.players.get(&id)
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<&mut Audio> {
        self.players.get_mut(&id)
    }

    pub fn add(&mut self, audio: Audio) -> u32 {
        let id = self.id_pool.get();
        self.players.insert(id, audio);
        id
    }

    pub fn remove(&mut self, id: u32) {
        if self.players.contains_key(&id) {
            self.players.remove(&id);
            self.id_pool.free(id);
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
        self.id_pool = IdPool::new();
    }

    pub fn list(&self) -> Vec<PlayerEntry> {
        self.players
            .iter()
            .map(
                |(id, audio)| PlayerEntry {
                    id: *id,
                    info: audio.info().clone()
                }
            )
            .collect()
    }

    pub fn play(&mut self, id: u32) {
        if let Some(audio) = self.players.get_mut(&id) {
            audio.play();
        }
    }

    pub fn stop(&mut self, id: u32) {
        if let Some(audio) = self.players.get_mut(&id) {
            audio.stop();
        }
    }

    pub fn pause(&mut self, id: u32) {
        if let Some(audio) = self.players.get_mut(&id) {
            audio.pause();
        }
    }
}