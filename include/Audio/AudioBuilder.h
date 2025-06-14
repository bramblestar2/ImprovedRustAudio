#pragma once

#include "rust/audio.h"
#include "types.h"

class AudioBuilder
{
public:
    AudioBuilder();
    AudioBuilder(AudioBuilder&& other);

    AudioBuilder& set_file(std::string file);
    AudioBuilder& set_loop(bool loop);
    AudioBuilder& set_speed(float speed);
    AudioBuilder& set_volume(float volume);
    AudioBuilder& set_fade_in(float fade_in);
    AudioBuilder& set_fade_out(float fade_out);
    AudioBuilder& set_start(float start);
    AudioBuilder& set_end(float end);



    rust::cxxbridge1::Box<rust_audio::AudioBuilder> get();

private:
    rust::cxxbridge1::Box<rust_audio::AudioBuilder> builder;
};