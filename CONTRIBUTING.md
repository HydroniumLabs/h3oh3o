# Contribution guidelines

First off, thank you for considering contributing to h3oh3o.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/HydroniumLabs/h3oh3o/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/HydroniumLabs/h3oh3o/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Useful Commands

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features
  ```

- Run tests:

  ```shell
  cd h3oh3o/h3tests
  mkdir -p build && cd build
  cmake .. -DCMAKE_BUILD_TYPE=Debug
  make
  make test
  ```

- Run benchmarks:

  ```shell
  mkdir -p build && cd build
  cmake .. -DCMAKE_BUILD_TYPE=Release
  make benchmarks
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo +nightly fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo +nightly fmt --all
  ```
