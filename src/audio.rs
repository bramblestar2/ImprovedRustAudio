use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use tracing::error;

#[derive(Clone)]
struct PlayerSettings {
    volume: f32,
    start_time: f32,
    end_time: f32,
    looped: bool,

    fade_in: f32,
    fade_out: f32,
    speed: f32
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            volume: 1.0,
            start_time: 0.0,
            end_time: 0.0,
            looped: false,
            fade_in: 0.0,
            fade_out: 0.0,
            speed: 1.0
        }
    }
}

impl PlayerSettings {
    pub fn set_volume(&mut self, volume: f32) -> &mut Self {
        self.volume = volume;
        self
    }

    pub fn set_loop(&mut self, looped: bool) -> &mut Self {
        self.looped = looped;
        self
    }

    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    pub fn set_fade_in(&mut self, fade_in: f32)-> &mut Self {
        self.fade_in = fade_in;
        self
    }

    pub fn set_fade_out(&mut self, fade_out: f32)-> &mut Self {
        self.fade_out = fade_out;
        self
    }

    pub fn set_start(&mut self, start_time: f32)-> &mut Self {
        self.start_time = start_time;
        self
    }

    pub fn set_end(&mut self, end_time: f32)-> &mut Self {
        self.end_time = end_time;
        self
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn looped(&self) -> bool {
        self.looped
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn fade_in(&self) -> f32 {
        self.fade_in
    }

    pub fn fade_out(&self) -> f32 {
        self.fade_out
    }

    pub fn start_time(&self) -> f32 {
        self.start_time
    }

    pub fn end_time(&self) -> f32 {
        self.end_time
    }
}

#[derive(Default, Clone)]
struct PlayerInfo {
    file: String,
}

pub struct Audio {
    sink: Sink,
    settings: PlayerSettings,
    info: PlayerInfo
}

impl Audio {
    pub fn new(handle: &OutputStreamHandle) -> Audio {
        Audio {
            sink: Sink::try_new(handle).unwrap(),
            settings: PlayerSettings::default(),
            info: PlayerInfo::default()
        }
    }

    pub fn clone_with_handle(&self, handle: &OutputStreamHandle) -> Audio {
        Audio {
            sink: Sink::try_new(handle).unwrap(),
            settings: self.settings.clone(),
            info: self.info.clone()
        }
    }


    // SETTINGS
    pub fn settings(&self) -> &PlayerSettings {
        &self.settings
    }

    pub fn set_file(&mut self, file: &str)-> &mut Self {
        self.info.file = file.to_string();
        self
    }

    pub fn file(&self) -> &str {
        &self.info.file
    }

    // CONTROLS
    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn play(&mut self, file: &str) {
        self.stop();
        
        match File::open(file) {
            Ok(file) => {
                match Decoder::new(BufReader::new(file)) {
                    Ok(source) => {
                        self.sink.set_volume(self.settings.volume);
                        self.sink.append(source);
                    },
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            },
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}