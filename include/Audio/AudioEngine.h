#pragma once

#include "rust/audio.h"

class AudioEngine {
public:
    AudioEngine();
    ~AudioEngine();

    

private:
    rust::cxxbridge1::Box<rust_audio::Engine> engine;
};