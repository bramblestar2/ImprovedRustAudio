use cxx::UniquePtr;
use cxx::SharedPtr;

use crate::engine::Engine;
use crate::audio::{Audio};
use crate::manager::{PlayerManager, PlayerEntry};
use crate::audio::{AudioBuilder, PlayerSettings, PlayerInfo};



#[cxx::bridge(namespace = "rust_audio")]
pub mod FFIAudio {
    extern "Rust" {
        type Engine;
        type Audio;
        type AudioBuilder;
        type PlayerManager;
        type PlayerEntry;
        type PlayerSettings;
        type PlayerInfo;

        fn json(self: &Engine) -> String;
        fn load(self: &mut Engine, filepath: &str);
        fn save(self: &mut Engine, filepath: &str);

        fn builder(engine: &mut Engine) -> Box<AudioBuilder>;
        fn create(engine: &mut Engine, builder: Box<AudioBuilder>) -> u32;
        fn remove(self: &mut Engine, id: u32);
        fn clear(self: &mut Engine);
        fn list(self: &Engine) -> Vec<PlayerEntry>;
        fn manager(self: &mut Engine) -> &mut PlayerManager;
        fn play(self: &mut Engine, id: u32);
        fn stop(self: &mut Engine, id: u32);
        fn pause(self: &mut Engine, id: u32);
    }

    extern "Rust" {
        fn set_file(self: &mut AudioBuilder, file: &str);
        fn set_loop(self: &mut AudioBuilder, loop_: bool);
        fn set_speed(self: &mut AudioBuilder, speed: f32);
        fn set_volume(self: &mut AudioBuilder, volume: f32);
        fn set_fade_in(self: &mut AudioBuilder, fade_in: f32);
        fn set_fade_out(self: &mut AudioBuilder, fade_out: f32);
        fn set_start(self: &mut AudioBuilder, start_time: f32);
        fn set_end(self: &mut AudioBuilder, end_time: f32);
    }

    extern "Rust" {
        fn id(self: &PlayerEntry) -> u32;
        fn info(self: &PlayerEntry) -> &PlayerInfo;

        fn file(self: &PlayerInfo) -> &str;
        fn settings(self: &PlayerInfo) -> &PlayerSettings;

        fn set_volume(self: &mut PlayerSettings, volume: f32) -> &mut PlayerSettings;
        fn set_loop(self: &mut PlayerSettings, looped: bool) -> &mut PlayerSettings;
        fn set_speed(self: &mut PlayerSettings, speed: f32) -> &mut PlayerSettings;
        fn set_fade_in(self: &mut PlayerSettings, fade_in: f32) -> &mut PlayerSettings;
        fn set_fade_out(self: &mut PlayerSettings, fade_out: f32) -> &mut PlayerSettings;
        fn set_start(self: &mut PlayerSettings, start_time: f32) -> &mut PlayerSettings;
        fn set_end(self: &mut PlayerSettings, end_time: f32) -> &mut PlayerSettings;

        fn volume(self: &PlayerSettings) -> f32;
        fn looped(self: &PlayerSettings) -> bool;
        fn speed(self: &PlayerSettings) -> f32;
        fn fade_in(self: &PlayerSettings) -> f32;
        fn fade_out(self: &PlayerSettings) -> f32;
        fn start_time(self: &PlayerSettings) -> f32;
        fn end_time(self: &PlayerSettings) -> f32;
    }

    extern "Rust" {
        fn new_engine() -> Box<Engine>;
        fn create_builder() -> Box<AudioBuilder>;
    }
}

fn new_engine() -> Box<Engine> {
    Box::new(Engine::new())
}

fn builder(engine: &mut Engine) -> Box<AudioBuilder> {
    Box::new(engine.builder())
}

fn create(engine: &mut Engine, builder: Box<AudioBuilder>) -> u32 {
    engine.create(*builder)
}

fn create_builder() -> Box<AudioBuilder> {
    Box::new(AudioBuilder::new())
}