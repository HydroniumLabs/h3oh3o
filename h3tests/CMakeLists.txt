cmake_minimum_required(VERSION 3.20)
project(h3oh3o-tests)
enable_testing()

# Include the H3OH3O library, fetch it if not locally available.
include(FetchContent)
FetchContent_Declare(
    h3oh3o
    # Insert here the tag of the targeted version
    GIT_REPOSITORY https://github.com/HydroniumLabs/h3oh3o.git
    FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(h3oh3o)

add_subdirectory(common)
add_subdirectory(tests)
add_subdirectory(benchmarks)
