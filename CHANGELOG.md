# Changelog

## [0.4.1] - 2023-08-18

### Changed
- switch from [rust-build.action](https://github.com/marketplace/actions/rust-release-binary) to [build-and-upload-rust-binary-to-github-releases](https://github.com/marketplace/actions/build-and-upload-rust-binary-to-github-releases) github actions for releasing the artifacts
- rename `RELEASE_NOTES.md` to `CHANGE_LOG.md`

## [0.4.0] - 2023-08-16

### Added
- add command line argument support
- add configuration file support

### Fix
- add missing Debug traits

### Refactor
- coding style improvements

## [0.3.0] - 2023-08-15

### Added
- monitoring feature
- display format for MQTTClient errors

### Refactor
- update error handling and module invocation in main

## [0.2.0] - 2023-08-14

### Added
- docker file for creating MQTT broker container
- instructions on how to use docker to test T.U.M.'s publishing feature
- MQTTClient crate to T.U.M. for handling MQTT transport

### Changed
- use T.U.M. as a binary crate instead of a library crate

## [0.1.0] - 2023-08-11

_First release._

### Added
- get resources for the system in a OS agnostic way
- package resource information in JSON format

### Changed
- update initial README.md
