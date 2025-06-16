#include <iostream>
#include "../include/Audio/AudioBuilder.h"

#include "../include/Audio/AudioEngine.h"

int main() {
    std::cout << "Hello, World!" << std::endl;

    AudioBuilder builder;
    
    builder.set_file("/home/jay/Downloads/powerUp.wav");
    
    AudioEngine engine;
    int id = engine.create(builder);

    std::cin.get();

    engine.play(id);

    std::cin.get();
    
    return 0;
}