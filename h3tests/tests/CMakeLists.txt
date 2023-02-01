# Macros and support code needed to build and add the tests
set(TEST_NUMBER 0)

if(WIN32)
    # Use bash (usually from Git for Windows) for piping results
    set(SHELL bash -c)
else()
    set(SHELL sh -c)
endif()

macro(add_test_common NAME SRCFILE)
    # Only register the test binary once.
    if(NOT TARGET ${NAME})
        add_executable(${NAME} ${SRCFILE} src/test.c)
        target_link_libraries(${NAME} PUBLIC h3oh3o::h3oh3o common)
    endif()

    math(EXPR TEST_NUMBER "${TEST_NUMBER}+1")
endmacro()

macro(add_unit_test NAME SRCFILE)
    add_test_common(${NAME} ${SRCFILE})
    add_test(NAME ${NAME}_test${TEST_NUMBER} COMMAND "$<TARGET_FILE:${NAME}>")
endmacro()

macro(add_unit_test_with_file NAME SRCFILE DATAFILE)
    add_test_common(${NAME} ${SRCFILE})
    set(DUMP_COMMAND "cat")

    add_test(
        NAME ${NAME}_test${TEST_NUMBER}
        COMMAND ${SHELL} "${DUMP_COMMAND} ${DATAFILE} | $<TARGET_FILE:${NAME}>"
    )
endmacro()

# ------------------------------------------------------------------------------

# Add each individual test
file(GLOB ALL_CENTERS data/bc*centers.txt)
foreach(FILE ${ALL_CENTERS})
    add_unit_test_with_file(testLatLngToCell src/testLatLngToCell.c ${FILE})
endforeach()

file(GLOB ALL_IC data/res*ic.txt)
foreach(FILE ${ALL_IC})
    add_unit_test_with_file(testCellToLatLng src/testCellToLatLng.c ${FILE})
endforeach()

add_unit_test(testGridDisk src/testGridDisk.c)
add_unit_test(testGridDisksUnsafe src/testGridDisksUnsafe.c)
add_unit_test(testGridRingUnsafe src/testGridRingUnsafe.c)
add_unit_test(testLatLng src/testLatLng.c)
add_unit_test(testCellToCenterChild src/testCellToCenterChild.c)
add_unit_test(testCellsToLinkedMultiPolygon src/testCellsToLinkedMultiPolygon.c)
add_unit_test(testPolygonToCells src/testPolygonToCells.c)
add_unit_test(testPolygonToCellsReported src/testPolygonToCellsReported.c)
add_unit_test(testCellToChildPos src/testCellToChildPos.c)
