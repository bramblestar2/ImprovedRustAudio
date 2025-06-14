#include "Audio/AudioBuilder.h"

AudioBuilder::AudioBuilder()
    : builder(rust_audio::create_builder())
{
}
AudioBuilder::AudioBuilder(AudioBuilder &&other)
    : builder(std::move(other.builder))
{
}

AudioBuilder &AudioBuilder::set_file(std::string file)
{
    this->builder->set_file(file);
    return *this;
}

AudioBuilder &AudioBuilder::set_loop(bool loop)
{
    this->builder->set_loop(loop);
    return *this;
}

AudioBuilder &AudioBuilder::set_speed(float speed)
{
    this->builder->set_speed(speed);
    return *this;
}

AudioBuilder &AudioBuilder::set_volume(float volume)
{
    this->builder->set_volume(volume);
    return *this;
}

AudioBuilder &AudioBuilder::set_fade_in(float fade_in)
{
    this->builder->set_fade_in(fade_in);
    return *this;
}

AudioBuilder &AudioBuilder::set_fade_out(float fade_out)
{
    this->builder->set_fade_out(fade_out);
    return *this;
}

AudioBuilder &AudioBuilder::set_start(float start)
{
    this->builder->set_start(start);
    return *this;
}

AudioBuilder &AudioBuilder::set_end(float end)
{
    this->builder->set_end(end);
    return *this;
}

rust::cxxbridge1::Box<rust_audio::AudioBuilder> AudioBuilder::get() 
{
    return std::move(builder);
}