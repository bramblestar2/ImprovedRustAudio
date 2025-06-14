#pragma once
#include <string>
#include <vector>

#include "rust/audio.h"
#include "types.h"
#include "AudioBuilder.h"

class AudioEngine {
public:
    AudioEngine();

    std::string json();
    void load(std::string filepath);
    void save(std::string filepath);

    AudioBuilder builder();

    uint32_t create(AudioBuilder builder);
    void remove(uint32_t id);
    void clear();
    std::vector<PlayerEntry> list();
    rust_audio::PlayerManager& manager();
    void play(uint32_t id);
    void stop(uint32_t id);
    void pause(uint32_t id);

private:
    rust::cxxbridge1::Box<rust_audio::Engine> engine;
};