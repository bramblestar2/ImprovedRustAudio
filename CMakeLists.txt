cmake_minimum_required(VERSION 3.20)
project(improved_rust_audio LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


find_package(PkgConfig REQUIRED)
pkg_check_modules(ALSA REQUIRED alsa)


add_custom_target(build_rust_audio
  COMMAND cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  COMMENT "🚀 Building Rust staticlib"
  VERBATIM
)

find_library(RUST_AUDIO_LIB
  NAMES improved_rust_audio
  PATHS ${CMAKE_CURRENT_SOURCE_DIR}/target/debug
)
if (NOT RUST_AUDIO_LIB)
  message(FATAL_ERROR "Could not find libimproved_rust_audio.so")
endif()

add_library(rust_audio SHARED IMPORTED)
set_target_properties(rust_audio PROPERTIES
  IMPORTED_LOCATION ${RUST_AUDIO_LIB}
)
add_dependencies(rust_audio build_rust_audio)

add_executable(improved_rust_audio
    src/main.cpp
    src/Audio/Audio.cpp
    src/Audio/AudioEngine.cpp
    src/Audio/AudioBuilder.cpp

    include/rust/audio.cc
)

target_include_directories(improved_rust_audio PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
target_link_libraries(improved_rust_audio PRIVATE 
    rust_audio
    ${CMAKE_DL_LIBS}
    ${ALSA_LIBRARIES}
)
add_dependencies(improved_rust_audio rust_audio)
