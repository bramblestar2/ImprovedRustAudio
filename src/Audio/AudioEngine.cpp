#include "../../include/Audio/AudioEngine.h"

AudioEngine::AudioEngine()
    : engine(rust_audio::new_engine())
{
}