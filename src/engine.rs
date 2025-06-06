use std::{collections::HashMap, sync::Arc};

use rodio::{OutputStream, OutputStreamHandle};

use crate::{audio::Audio, utils::IdPool};

struct PlayerManager {
    id_pool: IdPool,
    players: HashMap<u32, Vec<Audio>>
}

pub struct Engine {
    _stream: OutputStream,
    _handle: Arc<OutputStreamHandle>,

    player: PlayerManager
}

impl Engine {

}