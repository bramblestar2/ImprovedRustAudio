#include <iostream>
#include "../include/Audio/AudioBuilder.h"

#include "../include/Audio/AudioEngine.h"

int main() {
    std::cout << "Hello, World!" << std::endl;

    AudioBuilder builder;
    
    builder.set_file("/home/jay/Downloads/C418 - The Fighter.mp3");
    builder.set_loop(true);
    builder.set_volume(0.5);
    builder.set_end(20.0);
    builder.set_start(10.0);
    
    AudioEngine engine;
    int id = engine.create(builder);

    engine.play(id);

    std::cin.get();
    
    return 0;
}