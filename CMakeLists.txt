cmake_minimum_required(VERSION 3.20)

project(
    improved_rust_audio
    VERSION 0.1.0
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(improved_rust_audio SHARED
    src/Audio/Audio.cpp
    src/Audio/AudioEngine.cpp
)

target_include_directories(improved_rust_audio PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)