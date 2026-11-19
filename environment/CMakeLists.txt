cmake_minimum_required(VERSION 3.21)

project(environment LANGUAGES C CXX)

# ---------------------------------------------------------------------------
# Build options
# ---------------------------------------------------------------------------
option(HALITE_STATIC_RUNTIME
       "Link the C/C++ runtime statically where the platform supports it"
       ON)

# Use a sensible default when configured without a preset.
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
    set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
                 Debug Release RelWithDebInfo MinSizeRel)
endif()

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Match the historical Windows build (/MT) while allowing dynamic linking.
if(MSVC AND HALITE_STATIC_RUNTIME)
    set(CMAKE_MSVC_RUNTIME_LIBRARY
        "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

# ---------------------------------------------------------------------------
# Version header
# ---------------------------------------------------------------------------
set(HALITE_VERSION "" CACHE STRING
    "Override the engine version (defaults to git describe)")

if(NOT HALITE_VERSION)
    find_package(Git QUIET)
    if(Git_FOUND)
        execute_process(
            COMMAND "${GIT_EXECUTABLE}" describe --abbrev=3 --tags HEAD
            WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
            RESULT_VARIABLE _halite_git_result
            OUTPUT_VARIABLE _halite_git_version
            ERROR_QUIET
            OUTPUT_STRIP_TRAILING_WHITESPACE)
        if(_halite_git_result EQUAL 0 AND _halite_git_version)
            string(REPLACE "-" "." HALITE_VERSION "${_halite_git_version}")
            string(REGEX REPLACE "^v" "" HALITE_VERSION "${HALITE_VERSION}")
        endif()
    endif()
endif()

if(NOT HALITE_VERSION)
    set(HALITE_VERSION "0.0.0")
endif()

message(STATUS "Halite version: ${HALITE_VERSION}")
configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/version.hpp.in"
    "${CMAKE_CURRENT_BINARY_DIR}/generated/version.hpp"
    @ONLY)

# ---------------------------------------------------------------------------
# Dependencies
# ---------------------------------------------------------------------------
find_package(Threads REQUIRED)

# ---------------------------------------------------------------------------
# Halite
# ---------------------------------------------------------------------------
add_executable(halite main.cpp)

target_include_directories(halite PRIVATE
    "${CMAKE_CURRENT_BINARY_DIR}/generated"
    "${CMAKE_CURRENT_SOURCE_DIR}"
    "${CMAKE_CURRENT_SOURCE_DIR}/core"
    "${CMAKE_CURRENT_SOURCE_DIR}/core/mapgen"
    "${CMAKE_CURRENT_SOURCE_DIR}/networking")

if(MSVC)
    target_compile_definitions(halite PRIVATE
        _LIBCPP_COMPILER_MSVC=1
        _CRT_SECURE_NO_WARNINGS)
    target_compile_options(halite PRIVATE /W2)
else()
    target_compile_options(halite PRIVATE
        -Wall
        -Wno-sign-compare
        -Wno-unused-function
        -pedantic)
endif()

if(WIN32)
    target_compile_definitions(halite PRIVATE NOMINMAX)
endif()

if(HALITE_STATIC_RUNTIME AND NOT MSVC AND NOT APPLE)
    target_link_options(halite PRIVATE -static)
endif()

# Each source directory adds its files to the halite target.
add_subdirectory(tclap)
add_subdirectory(zstd-1.3.0)
add_subdirectory(core)
add_subdirectory(networking)

target_link_libraries(halite PRIVATE zstd tclap Threads::Threads)
