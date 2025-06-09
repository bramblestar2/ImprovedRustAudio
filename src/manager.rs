use std::collections::HashMap;

use crate::{audio::Audio, utils::IdPool};

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

    pub fn list(&self) -> Vec<(u32, &Audio)> {
        self.players.iter().map(|(id, audio)| (*id, audio)).collect()
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