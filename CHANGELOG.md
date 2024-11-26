# Mabe Changelog

This file contains a record of all notable changes to the `mabe` project.

## Unreleased

Nothing yet.

## 0.4.1 (November 25, 2024)

Added a 'CHANGELOG.md' file to the project.

## 0.4.0 (November 24, 2024)

Improved the documentation, changed the cargo feature named `colored` to `colorize`, made the `colorize` feature work without
the need for an external crate by using ANSI escape codes, and refined the API by enforcing the use of the `error` attribute on
all variants of the enum and prohibiting attributes form being used more than once.

## 0.3.1 (November 18, 2024)

Added a new helper function and interpolated the `write` macro in the generated implementations for easier debugging.

## 0.3.0 (November 17, 2024)

Created a cargo feature called `colored`, so that error, reason, and solution message prefixes can be printed with colors.

## 0.2.1 (November 16, 2024)

Moved the utility functions to the `mabe/` directory and removed the `mabe_utils/` directory from the project.

## 0.2.0 (November 16, 2024)

Enabled interpolation of variant fields in attribute messages.

## 0.1.2 (October 26, 2024)

Restructured the project by creating a the `mabe/` and `mabe_utils/` directories for the `Mabe` derive macro and the utility
functions respectively.

## 0.1.1 (October 26, 2024)

Changed the configurations of the `Cargo.toml` file to exclude the `node_modules/` directory from the crate.

## 0.1.0 (October 26, 2024)

Initial release.
