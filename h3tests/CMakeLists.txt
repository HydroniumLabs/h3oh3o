cmake_minimum_required(VERSION 3.20)
project(h3oh3o-tests)
enable_testing()

# Include the CH3O library, fetch it if not locally available.
include(FetchContent)
FetchContent_Declare(
    h3oh3o
    # FIXME
    GIT_REPOSITORY ssh://git@github.com/HydroniumLabs/h3oh3o.git
    FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(h3oh3o)

add_subdirectory(common)
add_subdirectory(tests)
add_subdirectory(benchmarks)
