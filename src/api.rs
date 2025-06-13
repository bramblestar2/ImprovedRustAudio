use cxx::UniquePtr;
use cxx::SharedPtr;

use crate::engine::Engine;
use crate::audio::Audio;
use crate::manager::{PlayerManager, PlayerEntry};
use crate::audio::AudioBuilder;


#[cxx::bridge(namespace = "rust_audio")]
mod FFIAudio {


    

    extern "Rust" {
        type Engine;
        type Audio;
        type AudioBuilder;
        type PlayerManager;
        type PlayerEntry;

        fn new_engine() -> Box<Engine>;

        fn json(self: &Engine) -> String;
        fn load(self: &mut Engine, filepath: &str);
        fn save(self: &mut Engine, filepath: &str);

        fn builder(engine: &mut Engine) -> Box<AudioBuilder>;
        fn remove(self: &mut Engine, id: u32);
        fn clear(self: &mut Engine);
        fn list(self: &Engine) -> Vec<PlayerEntry>;
        fn manager(self: &mut Engine) -> &mut PlayerManager;
        fn play(self: &mut Engine, id: u32);
        fn stop(self: &mut Engine, id: u32);
        fn pause(self: &mut Engine, id: u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn new_engine() -> Box<Engine> {
    Box::new(Engine::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn builder(engine: &mut Engine) -> Box<AudioBuilder> {
    Box::new(engine.builder())
}