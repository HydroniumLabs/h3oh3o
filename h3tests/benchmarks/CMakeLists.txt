include(CheckIPOSupported)
check_ipo_supported(RESULT lto_supported OUTPUT error)
if(lto_supported)
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_RELEASE TRUE)
else()
    message(WARNING "IPO is not supported: ${error}")
endif()

add_custom_target(benchmarks)

macro(add_benchmark NAME SRCFILE)
    add_executable(${NAME} ${SRCFILE})
    target_link_libraries(${NAME} PUBLIC h3oh3o::h3oh3o)
    set_property(TARGET ${NAME} PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)

    add_custom_target(bench_${NAME} COMMAND $<TARGET_FILE:${NAME}>)

    add_dependencies(benchmarks bench_${NAME})
endmacro()

add_benchmark(benchmarkCellsToLinkedMultiPolygon  benchmarkCellsToLinkedMultiPolygon.c)
add_benchmark(benchmarkbenchmarkDirectedEdge benchmarkDirectedEdge.c)
add_benchmark(benchmarkGridDiskCells benchmarkGridDiskCells.c)
add_benchmark(benchmarkGridPathCells benchmarkGridPathCells.c)
add_benchmark(benchmarkH3Api benchmarkH3Api.c)
add_benchmark(benchmarkIsValidCell benchmarkIsValidCell.c)
add_benchmark(benchmarkPolygonToCells benchmarkPolygonToCells.c)
add_benchmark(benchmarkVertex benchmarkVertex.c)
