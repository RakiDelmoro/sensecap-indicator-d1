cmake_minimum_required(VERSION 3.10)
project(sensecap-simulator C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Find SDL2
find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

# Set LVGL configuration
set(LV_CONF_BUILD_DISABLE_EXAMPLES 1)
set(LV_CONF_BUILD_DISABLE_DEMOS 1)
set(LV_CONF_INCLUDE_SIMPLE 1)
set(LV_LVGL_H_INCLUDE_SIMPLE 1)

# LVGL configuration path
set(LV_CONF_PATH ${CMAKE_CURRENT_SOURCE_DIR}/lv_conf.h CACHE STRING "" FORCE)

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/lvgl
    ${CMAKE_CURRENT_SOURCE_DIR}/lvgl/src
    ${CMAKE_CURRENT_SOURCE_DIR}/lvgl/src/hal/sdl
    ${CMAKE_CURRENT_SOURCE_DIR}/ui
    ${CMAKE_CURRENT_SOURCE_DIR}/ui/screens
    ${CMAKE_CURRENT_SOURCE_DIR}/ui/components
    ${CMAKE_CURRENT_SOURCE_DIR}/ui/images
)

# Collect UI source files
file(GLOB UI_SOURCES
    ui/*.c
    ui/screens/*.c
    ui/components/*.c
    ui/images/*.c
)

# Collect LVGL source files
file(GLOB_RECURSE LVGL_SOURCES 
    lvgl/src/*.c
)

# Rust library path
set(RUST_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../simulator-backend/target/release)
set(RUST_LIB ${RUST_LIB_DIR}/libsimulator_backend.a)

# Create executable
add_executable(sensecap-simulator
    src/main.c
    ${UI_SOURCES}
    ${LVGL_SOURCES}
)

# Ensure Rust library is built before C executable
add_custom_command(
    OUTPUT ${RUST_LIB}
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/../simulator-backend
    COMMENT "Building Rust simulator-backend library"
)

add_custom_target(rust_backend DEPENDS ${RUST_LIB})
add_dependencies(sensecap-simulator rust_backend)

# Link libraries
target_link_libraries(sensecap-simulator PRIVATE
    ${RUST_LIB}
    ${SDL2_LIBRARIES}
    m
    pthread
    dl
)

# Compiler flags
target_compile_options(sensecap-simulator PRIVATE
    -DLV_CONF_INCLUDE_SIMPLE=1
    -DLV_LVGL_H_INCLUDE_SIMPLE=1
    -DLV_USE_SDL=1
)

# Print status
message(STATUS "SDL2 include dirs: ${SDL2_INCLUDE_DIRS}")
message(STATUS "SDL2 libraries: ${SDL2_LIBRARIES}")
message(STATUS "UI sources: ${UI_SOURCES}")
