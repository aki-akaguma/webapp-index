# Changelog: webapp-index

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
* update crate: dioxus(0.7.6)


## [0.1.4] (2026-04-21)
### Added
* `AppDetailDialog` component
* `struct DescMsg`
* `tracing`

### Changed
* replace hardcoded paths: `/.../config.toml` to using an environment variable
* `<dialog>` elements open control
* I reorganized the dependencies in `Cargo.toml`.
* `version_compare` to `semver` crate

### Fixed
* `Version::parse(version_s.as_str())?`

## [0.1.3] (2026-04-15)
### Changed
* renamed app: `webapp-index` to `webapp-akiapp`

### Fixed
* android icon
* download link address
* link to `Route::`
* android wva
* `dx bundle --desktop --release --package-types appimage` on `Makefile`

## [0.1.2] (2026-04-14)
### Added
* download dialog

### Changed
* `dioxus_logger` to `dioxus::logger`
* backend interface: return type of `list_apps()`

## [0.1.1] (2026-04-11)
### Added
* `backends::Config` on `config.toml`
* router of dioxus

### Changed
* update crates: dioxus(0.7.5)
* put the app name in the APK download URL

## [0.1.0] (2026-04-03)
### Added
* first commit

[Unreleased]: https://github.com/aki-akaguma/webapp-akiapp/compare/v0.1.4..HEAD
[0.1.4]: https://github.com/aki-akaguma/webapp-akiapp/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/webapp-akiapp/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/webapp-akiapp/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/webapp-akiapp/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/webapp-akiapp/releases/tag/v0.1.0
