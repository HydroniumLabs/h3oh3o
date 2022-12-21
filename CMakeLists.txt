cmake_minimum_required(VERSION 3.20)
project(h3oh3o)

include(GNUInstallDirs)

include(CheckIPOSupported)
check_ipo_supported(RESULT lto_supported OUTPUT error)
if(lto_supported)
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_RELEASE TRUE)
else()
    message(WARNING "IPO is not supported: ${error}")
endif()

# Install Corrosion if missing, used expose Rust crates as CMake targets.
include(FetchContent)
FetchContent_Declare(Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3.0
    FIND_PACKAGE_ARGS
)

FetchContent_MakeAvailable(Corrosion)

# Expose the h3oh3o crate.
set(CRATE h3oh3o)
corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES ${CRATE})
# Add an alias to support both find_package and FetchContent
add_library(${CRATE}::${CRATE} ALIAS ${CRATE})

# Header generated with cbindgen.
set(CODEGEN_OUTPUT_DIR ${CMAKE_CURRENT_BINARY_DIR}/gen/)
# Pass the env var to cargo (for `build.rs`)
corrosion_set_env_vars(${CRATE} "CH3O_EXPORT_DIR=${CODEGEN_OUTPUT_DIR}")

# Add include directory for the generated header.
target_include_directories(${CRATE}
    INTERFACE
    "$<BUILD_INTERFACE:${CODEGEN_OUTPUT_DIR}>"
    "$<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>"
)

# Install the target and create export-set.
install(TARGETS ${CRATE}
    EXPORT ${CRATE}Config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

export(TARGETS ${CRATE}
    NAMESPACE ${CRATE}::
    FILE "${CMAKE_CURRENT_BINARY_DIR}/${CRATE}Config.cmake"
)

install(EXPORT ${CRATE}Config
    DESTINATION "${CMAKE_INSTALL_DATADIR}/${CRATE}/cmake"
    NAMESPACE ${CRATE}::
)
