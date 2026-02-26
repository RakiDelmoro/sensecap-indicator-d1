cmake_minimum_required(VERSION 3.20)

include($ENV{IDF_PATH}/tools/cmake/project.cmake)

# Set project name
project(sensecap-indicator-d1)

# Include the Rust component
set(EXTRA_COMPONENT_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/components")

# Register the main component
idf_component_register(
    SRCS
    INCLUDE_DIRS
)
