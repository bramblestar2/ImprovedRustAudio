#include "Audio/AudioEngine.h"

#include <algorithm>

AudioEngine::AudioEngine()
    : engine(rust_audio::new_engine())
{
}

std::string AudioEngine::json()
{
    return std::string(engine->json().c_str());
}


void AudioEngine::load(std::string filepath) {
    engine->load(filepath);
}

void AudioEngine::save(std::string filepath) {
    engine->save(filepath);
}

AudioBuilder AudioEngine::builder() {
    return AudioBuilder();
}

uint32_t AudioEngine::create(AudioBuilder& builder) {
    return rust_audio::create(*engine, *builder);
}

void AudioEngine::remove(uint32_t id) {
    engine->remove(id);
}

void AudioEngine::clear() {
    engine->clear();
}

std::vector<PlayerEntry> AudioEngine::list() {

    std::vector<PlayerEntry> list;

    for (auto& entry : engine->list()) {
        list.emplace_back(entry);
    }

    return list;
}

rust_audio::PlayerManager& AudioEngine::manager() {
    return engine->manager();
}

void AudioEngine::play(uint32_t id) {
    engine->play(id);
}

void AudioEngine::stop(uint32_t id) {
    engine->stop(id);
}

void AudioEngine::pause(uint32_t id) {
    engine->pause(id);
}
