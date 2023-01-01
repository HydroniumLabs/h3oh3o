# h3oh3o — H3 on H3O

[![CI Status](https://github.com/HydroniumLabs/h3oh3o/actions/workflows/ci.yml/badge.svg)](https://github.com/HydroniumLabs/h3oh3oh3o/actions)
[![License](https://img.shields.io/badge/license-BSD-green)](https://opensource.org/licenses/BSD-3-Clause)

## Design

This crate wraps the H3O library and expose a C API that can be used as a drop
in replacement for the H3 reference implementation.

Note that while the exposed API itself exacly matches the reference one, from a
behavioral point of view there are some differences (that shouldn't matter in
most cases but still: be aware of it).

For instances:
- when a function fails, the error code may differ between `h3` and `h3oh3o`
- due to the current implementation of h3's `compactCells`, some duplicates in
  the input may go undetected: `h3oh3o` will detect them and returns an error.
- `stringToH3` does no validity check on its input (beyond "is it an integer"),
  whereas `h3oh3o` ensure the parsed index's validity.
- when the resolution given to `cellToChildrenSize` is coarser than the cell's
  one, `h3` returns an error where `h3oh3o` returns a count of `0`.
- …

## Usage

To use `h3oh3o` as a C library in an external project, simply add the following
snippet to your `CMakeLists.txt`

```cmake
# Include the H3OH3O library, fetch it if not locally available.
include(FetchContent)
FetchContent_Declare(
    h3oh3o
    GIT_REPOSITORY https://github.com/HydroniumLabs/h3oh3o.git
    FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(h3oh3o)
```

And then

```cmake
target_link_libraries(your_target PUBLIC h3oh3o::h3oh3o)
```

## License

[BSD 3-Clause](./LICENSE)
