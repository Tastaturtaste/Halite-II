# Only the zstd components used by the engine are built.
add_library(zstd STATIC)

target_include_directories(zstd
    SYSTEM PUBLIC
        "${CMAKE_CURRENT_SOURCE_DIR}"
    PRIVATE
        "${CMAKE_CURRENT_SOURCE_DIR}/common"
        "${CMAKE_CURRENT_SOURCE_DIR}/compress"
        "${CMAKE_CURRENT_SOURCE_DIR}/decompress")

target_compile_definitions(zstd PRIVATE ZSTD_MULTITHREAD=1)
target_link_libraries(zstd PUBLIC Threads::Threads)

# Keep the vendored C code on a stable language version. MSVC's C standard
# support varies by toolset version, so leave its default in place.
if(NOT MSVC)
    set_target_properties(zstd PROPERTIES
        C_STANDARD 99
        C_STANDARD_REQUIRED ON
        C_EXTENSIONS OFF)
endif()

# zstd is vendored third-party code; keep its warnings out of our build output.
if(MSVC)
    target_compile_options(zstd PRIVATE /W0)
else()
    target_compile_options(zstd PRIVATE -w)
endif()

add_subdirectory(common)
add_subdirectory(compress)
add_subdirectory(decompress)
