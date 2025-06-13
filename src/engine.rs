use std::{collections::HashMap, sync::Arc};

use rodio::{OutputStream, OutputStreamHandle};

use crate::{audio::{Audio, AudioBuilder}, manager::PlayerManager, utils::IdPool};



pub struct Engine {
    _stream: OutputStream,
    _handle: Arc<OutputStreamHandle>,

    player: PlayerManager
}

impl Engine {
    pub fn new() -> Engine {
        let (stream, handle) = OutputStream::try_default().unwrap();
        Engine {
            _stream: stream,
            _handle: Arc::new(handle),
            player: PlayerManager::new()
        }
    }

    pub fn json(&self) -> String {
        self.player.json()
    }

    pub fn load(&mut self, filepath: &str) {
        self.player = PlayerManager::load(filepath, &self._handle);
    }

    pub fn save(&mut self, filepath: &str) {
        self.player.save(filepath);
    }

    pub fn get_handle(&self) -> Arc<OutputStreamHandle> {
        self._handle.clone()
    }

    pub fn get_player(&mut self, id: u32) -> Option<&Audio> {
        self.player.get_player(id)
    }

    pub fn get_player_mut(&mut self, id: u32) -> Option<&mut Audio> {
        self.player.get_player_mut(id)
    }

    pub fn builder(&mut self) -> AudioBuilder {
        AudioBuilder::new()
    }

    pub fn create(&mut self, builder: AudioBuilder) -> u32 {
        let id = self.player.add(builder.build(&self._handle));
        id
    }

    pub fn remove(&mut self, id: u32) {
        self.player.remove(id);
    }

    pub fn clear(&mut self) {
        self.player.clear();
    }

    pub fn list(&self) -> Vec<(u32, &Audio)> {
        self.player.list()
    }

    pub fn manager(&mut self) -> &mut PlayerManager {
        &mut self.player
    }
}