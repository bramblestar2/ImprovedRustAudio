#pragma once

#include <string>
#include <cstdint>

#include "rust/audio.h"

struct PlayerSettings {
    PlayerSettings() = default;
    PlayerSettings(const rust_audio::PlayerSettings& settings) {
        volume = settings.volume();
        start_time = settings.start_time();
        end_time = settings.end_time();
        looped = settings.looped();
        fade_in = settings.fade_in();
        fade_out = settings.fade_out();
        speed = settings.speed();
    }

    float volume;
    float start_time;
    float end_time;
    bool looped;
    float fade_in;
    float fade_out;
    float speed;
};

struct PlayerInfo {
    PlayerInfo() = default;
    PlayerInfo(const rust_audio::PlayerInfo& info) {
        file = std::string(info.file());
        settings = PlayerSettings(info.settings());
    }

    std::string file;
    PlayerSettings settings;
};

struct PlayerEntry {
    PlayerEntry() = default;
    PlayerEntry(const rust_audio::PlayerEntry& entry) {
        id = entry.id();
        info = PlayerInfo(entry.info());
    }

    uint32_t id;
    PlayerInfo info;
};

