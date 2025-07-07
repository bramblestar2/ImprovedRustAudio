cmake_minimum_required(VERSION 3.20)
project(improved_rust_audio LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_BUILD_FLAGS "")
    set(CARGO_LIB_PATH "debug")
else()
    set(CARGO_BUILD_FLAGS "--release")
    set(CARGO_LIB_PATH "release")
endif()

find_package(PkgConfig REQUIRED)

if (UNIX AND NOT APPLE)
    pkg_check_modules(ALSA REQUIRED alsa)
endif()

find_package(nlohmann_json CONFIG REQUIRED)

add_custom_target(build_rust_audio
    COMMAND cargo build ${CARGO_BUILD_FLAGS} --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building Rust staticlib"
    VERBATIM
)

find_library(RUST_AUDIO_LIB
    NAMES improved_rust_audio
    PATHS ${CMAKE_CURRENT_SOURCE_DIR}/target/${CARGO_LIB_PATH}
)

execute_process(
    COMMAND cargo build ${CARGO_BUILD_FLAGS} --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    RESULT_VARIABLE _cargo_result
)

if (NOT _cargo_result EQUAL 0)
    message(FATAL_ERROR "Could not build improved_rust_audio library (exit code: ${_cargo_result})")
endif()

if (NOT RUST_AUDIO_LIB)
    message(FATAL_ERROR "Could not find improved_rust_audio library")
endif()

add_library(rust_audio STATIC IMPORTED GLOBAL)
set_target_properties(rust_audio PROPERTIES
    IMPORTED_LOCATION ${RUST_AUDIO_LIB}
    IMPORTED_IMPLIB ${RUST_AUDIO_LIB}
)
add_dependencies(rust_audio build_rust_audio)

add_library(improved_rust_audio
    # src/main.cpp
    src/Audio/Audio.cpp
    src/Audio/AudioEngine.cpp
    src/Audio/AudioBuilder.cpp

    include/rust/audio.cc
)

target_include_directories(improved_rust_audio PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)
target_link_libraries(improved_rust_audio PUBLIC 
    rust_audio
    nlohmann_json::nlohmann_json
    ${CMAKE_DL_LIBS}
    ${ALSA_LIBRARIES}
)

if(WIN32)
    target_link_libraries(improved_rust_audio PUBLIC 
        Userenv 
        Propsys 
        windowsapp 
        ntdll
    )
endif()

add_dependencies(improved_rust_audio rust_audio)