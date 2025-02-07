# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.4.0] - 2025-02-07

### Added
- Implement `Eq` for more suitable types.

### Changed
- [breaking-change] Updated to use embedded-hal v1.
- Update usage of chrono::NaiveDate to use non-panic methods.
- Updated MSRV to version 1.75.0.

## [0.3.0] - 2022-08-16

### Added
- Implement `Eq` for suitable types.

### Changed
- [breaking-change] Adapted to `rtcc` 0.3.
- Updated MSRV to version 1.56.0.

## [0.2.1] - 2021-09-08

### Fixed
- Error when converting hours between 24H mode and AM/PM. Thanks to @ppelleti.

## [0.2.0] - 2020-02-09

### Changed
- Changed `get_datetime()` and `set_datetime()` parameter from `DateTime`
  to `chrono::NaiveDateTime`.

### Added
- Methods to set and get date and time using `chrono::NaiveDate` and `chrono::NaiveTime`:
    - `get_time()`
    - `set_time()`
    - `get_date()`
    - `set_date()`
- `chrono` dependency.

### Removed
- `DateTime` data structure was replaced by `chrono::NaiveDateTime`.

## [0.1.0] - 2019-09-15

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future. All changes will be documented in this
CHANGELOG.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/mcp794xx-rs/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/eldruin/mcp794xx-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/eldruin/mcp794xx-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/eldruin/mcp794xx-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/mcp794xx-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/mcp794xx-rs/releases/tag/v0.1.0
