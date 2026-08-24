# Changelog

This project is licensed under the [Apache License (Version 2.0)](LICENSE). The format of this file (and the project as a whole) follows [Phased Versioning](https://phased-versioning.koseka.net).

## v1-alpha.1 (2026-08-24)

- Fixed a bug where escaped curly braces in `error` messages were misparsed when the message contained non-ASCII characters.
- Terminal-width wrapping now counts characters instead of bytes, so non-ASCII error messages wrap correctly.
- Invalid inputs to the `Error` derive macro are now reported as spanned compile errors that point at the offending item, instead of panics.
- Replaced the unmaintained `term_size` dependency with `terminal_size`, and updated the vulnerable dependencies.
- Documented that the `main` attribute doesn't support async entry points, and that the `Error` derive macro reserves the `debug()` and `error()` method names.

## v1-alpha.0 (2026-02-18)

First stable version.
