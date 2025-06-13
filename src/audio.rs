use std::{fmt::Debug, fs::File, io::BufReader, time::Duration};

use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSettings {
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    file: String,
    settings: PlayerSettings
}

impl PlayerInfo {
    pub fn file(&self) -> &str {
        &self.file
    }
    
    pub fn settings(&self) -> &PlayerSettings {
        &self.settings
    }
}

#[derive(Serialize)]
pub struct Audio {
    #[serde(skip)]
    sink: Sink,
    info: PlayerInfo
}

impl Debug for Audio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Audio")
            .field("file", &self.info.file)
            .field("settings", &self.info.settings)
            .finish()
    }
}

impl Audio {
    pub fn new(handle: &OutputStreamHandle) -> Audio {
        Audio {
            sink: Sink::try_new(handle).unwrap(),
            info: PlayerInfo::default()
        }
    }

    pub fn clone_with_handle(&self, handle: &OutputStreamHandle) -> Audio {
        Audio {
            sink: Sink::try_new(handle).unwrap(),
            info: self.info.clone()
        }
    }

    pub fn from_info(info: PlayerInfo, handle: &OutputStreamHandle) -> Result<Audio, ()> {
        Ok(Audio {
            sink: Sink::try_new(handle).unwrap(),
            info
        })
    }


    // SETTINGS
    pub fn info(&self) -> &PlayerInfo {
        &self.info
    }

    pub fn info_mut(&mut self) -> &mut PlayerInfo {
        &mut self.info
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

    pub fn play(&mut self) {
        self.stop();

        let file = self.info.file.clone();
        
        match File::open(file) {
            Ok(file) => {
                match Decoder::new(BufReader::new(file)) {
                    Ok(source) => {
                        let mut source: Box<dyn Source<Item = i16> + Send> = Box::new(source);

                        let start_time = self.info.settings.start_time;
                        let end_time = self.info.settings.end_time;
                        let fade_in = self.info.settings.fade_in;
                        let fade_out = self.info.settings.fade_out;

                        if start_time > 0.0 {
                            source = Box::new(source.skip_duration(Duration::from_secs_f32(start_time)));
                        }

                        if end_time > start_time {
                            let take_duration = Duration::from_secs_f32(end_time - start_time);
                            source = Box::new(source.take_duration(take_duration));
                        }

                        if fade_in > 0.0 {
                            source = Box::new(source.fade_in(Duration::from_secs_f32(fade_in)));
                        }

                        if fade_out > 0.0 {
                            source = Box::new(source.fade_out(Duration::from_secs_f32(fade_out)));
                        }

                        self.sink.set_volume(self.info.settings.volume);
                        self.sink.set_speed(self.info.settings.speed);

                        if self.info.settings.looped {
                            self.sink.append(source.repeat_infinite());
                        } else {
                            self.sink.append(source);
                        }
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

#[derive(Debug, Clone, Default)]
pub struct AudioBuilder {
    info: PlayerInfo
}

impl AudioBuilder {
    pub fn new() -> AudioBuilder {
        AudioBuilder {
            info: PlayerInfo::default()
        }
    }

    pub fn set_file(mut self, file: &str)-> Self {
        self.info.file = file.to_string();
        self
    }

    pub fn set_loop(mut self, looped: bool) -> Self {
        self.info.settings.set_loop(looped);
        self
    }

    pub fn set_speed(mut self, speed: f32) -> Self {
        self.info.settings.set_speed(speed);
        self
    }

    pub fn set_volume(mut self, volume: f32) -> Self {
        self.info.settings.set_volume(volume);
        self
    }

    pub fn set_fade_in(mut self, fade_in: f32)-> Self {
        self.info.settings.set_fade_in(fade_in);
        self
    }

    pub fn set_fade_out(mut self, fade_out: f32)-> Self {
        self.info.settings.set_fade_out(fade_out);
        self
    }

    pub fn set_start(mut self, start_time: f32)-> Self {
        self.info.settings.set_start(start_time);
        self
    }

    pub fn set_end(mut self, end_time: f32)-> Self {
        self.info.settings.set_end(end_time);
        self
    }

    pub fn set_settings(mut self, settings: PlayerSettings)-> Self {
        self.info.settings = settings;
        self
    }

    pub fn set_info(mut self, info: PlayerInfo)-> Self {
        self.info = info;
        self
    }

    pub fn build(self, handle: &OutputStreamHandle) -> Audio {
        Audio {
            sink: Sink::try_new(handle).unwrap(),
            info: self.info
        }
    }
}