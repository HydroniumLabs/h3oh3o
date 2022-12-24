add_library(common
    algos.c
    baseCells.c
    bbox.c
    coordijk.c
    h3Index.c
    iterators.c
    latLng.c
    linkedGeo.c
    utility.c
)
target_link_libraries(common PUBLIC h3oh3o::h3oh3o)
target_include_directories(common PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
